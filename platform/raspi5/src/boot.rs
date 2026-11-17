use core::{panic::PanicInfo, ptr, slice};
use crust_kernel::{
    Context, Numeric, m_native__kernel as k, m_native__memory as m,
    m_native__vfs as v, m_native__thx as thx, m_native__start as start,
};
use crate::memory::BoardMemory;
use m::TetoMemory;

include!(concat!(env!("OUT_DIR"), "/abi.rs"));

const POOL_SIZE: usize = 80 * 1024 * 1024;
const ROOT_LIMIT: usize = 16 * 1024 * 1024;
const INVALID: f64 = u32::MAX as f64;
type Kernel<'a> = Context<BoardMemory<'a>>;

extern "C" {
    fn rei_platform_allocate(size: usize) -> *mut u8;
    fn rei_console_char(value: u8);
    fn rei_console_poll();
    fn rei_console_idle() -> !;
    fn rei_panic(message: *const u8) -> !;
    #[cfg(not(feature = "sd-root"))]
    static rei_root_start: u8;
    #[cfg(not(feature = "sd-root"))]
    static rei_root_end: u8;
    #[cfg(feature = "sd-root")]
    fn rei_sd_root_open(size: *mut u32) -> i32;
    #[cfg(feature = "sd-root")]
    fn rei_sd_root_read(buffer: *mut u8, length: u32) -> i32;
    #[cfg(feature = "sd-root")]
    fn rei_sd_root_close() -> i32;
}

fn print(text: &[u8]) {
    for &byte in text { unsafe { rei_console_char(byte) }; }
}

fn poll() { unsafe { rei_console_poll() }; }

#[panic_handler]
fn panic(info: &PanicInfo<'_>) -> ! {
    use core::fmt::Write;
    struct Console;
    impl Write for Console {
        fn write_str(&mut self, value: &str) -> core::fmt::Result {
            print(value.as_bytes());
            Ok(())
        }
    }
    let _ = writeln!(Console, "RUSTY-REI: {info}");
    unsafe { rei_panic(b"REI: RUST KERNEL PANIC\n\0".as_ptr()) }
}

fn micros() -> i128 {
    let ticks: u64;
    let frequency: u64;
    unsafe {
        core::arch::asm!("mrs {ticks}, cntpct_el0", "mrs {frequency}, cntfrq_el0",
            ticks = out(reg) ticks, frequency = out(reg) frequency,
            options(nomem, nostack, preserves_flags));
    }
    assert_ne!(frequency, 0, "timer frequency");
    (ticks as i128 * 1_000_000) / frequency as i128
}

fn status(value: f64, message: &str) { assert!(value == 0.0, "{message}: {value}"); }
fn reserve(ctx: &Kernel<'_>, mem: TetoMemory, length: usize) -> f64 {
    let address = k::tetoImageReserve(ctx, mem, length as f64);
    assert!(address != INVALID, "image reserve");
    address
}
fn release(ctx: &Kernel<'_>, mem: TetoMemory, at: f64, length: usize) {
    status(k::tetoImageRelease(ctx, mem, at, length as f64), "image release");
}
fn copy(ctx: &Kernel<'_>, mem: TetoMemory, at: f64, bytes: &[u8]) {
    for (i, &byte) in bytes.iter().enumerate() {
        m::storeU8(ctx, mem, at + i as f64, Numeric::Number(byte as f64));
    }
}

#[cfg(not(feature = "sd-root"))]
fn root_image(ctx: &Kernel<'_>, mem: TetoMemory) -> (f64, usize) {
    let begin = ptr::addr_of!(rei_root_start);
    let end = ptr::addr_of!(rei_root_end);
    let length = (end as usize).checked_sub(begin as usize).expect("root symbols");
    assert!(length > 0 && length <= ROOT_LIMIT, "embedded root length");
    let at = reserve(ctx, mem, length);
    // The linker emits the entire root archive between these symbols.
    let bytes = unsafe { slice::from_raw_parts(begin, length) };
    copy(ctx, mem, at, bytes);
    (at, length)
}

#[cfg(feature = "sd-root")]
fn root_image(ctx: &Kernel<'_>, mem: TetoMemory) -> (f64, usize) {
    print(b"REI: LOADING SD ROOT.VFS\n");
    let mut length = 0u32;
    assert_eq!(unsafe { rei_sd_root_open(&mut length) }, 0, "SD root open");
    let length = length as usize;
    assert!(length > 0 && length <= ROOT_LIMIT, "SD root length");
    let at = reserve(ctx, mem, length);
    let mut chunk = [0u8; 4096];
    let mut offset = 0;
    while offset < length {
        let count = (length - offset).min(chunk.len());
        if unsafe { rei_sd_root_read(chunk.as_mut_ptr(), count as u32) } != 0 {
            unsafe { rei_sd_root_close() };
            panic!("SD root read");
        }
        copy(ctx, mem, at + offset as f64, &chunk[..count]);
        offset += count;
        poll();
    }
    assert_eq!(unsafe { rei_sd_root_close() }, 0, "SD root close");
    print(b"REI: SD ROOT READ PASS\n");
    (at, length)
}

fn component(ctx: &Kernel<'_>, mem: TetoMemory, parent: f64,
             scratch: f64, name: &[u8]) -> f64 {
    assert!(name.len() <= 256);
    copy(ctx, mem, scratch, name);
    let inode = v::tetoVfsLookup(ctx, mem, parent, scratch, name.len() as f64);
    assert!(inode != 0.0 && inode != INVALID, "VFS lookup");
    inode
}

// Launch the same real mikuOS fixture as the C hardware target.
fn run_hello(ctx: &Kernel<'_>, mem: TetoMemory, root: f64) {
    let scratch = reserve(ctx, mem, 256);
    let etc = component(ctx, mem, root, scratch, b"etc");
    let motd = component(ctx, mem, etc, scratch, b"motd");
    let length = v::tetoVfsFileSize(ctx, mem, motd);
    assert!(length > 0 && length <= 256, "MOTD length");
    assert_eq!(v::tetoVfsReadData(ctx, mem, motd, 0, scratch, length as f64), length as f64);
    for i in 0..length { unsafe { rei_console_char(m::loadU8(ctx, mem, scratch + i as f64) as u8) }; }
    print(b"\nREI: MIKUOS MOTD READ PASS\n");
    let usr = component(ctx, mem, root, scratch, b"usr");
    let bin = component(ctx, mem, usr, scratch, b"bin");
    let hello = component(ctx, mem, bin, scratch, b"hello-rv64.39");
    let size = v::tetoVfsFileSize(ctx, mem, hello);
    release(ctx, mem, scratch, 256);
    assert!(size > 0 && size < 1024 * 1024, "hello length");
    let size = size as usize;
    status(k::tetoHartInit(ctx, mem, 0.0, 0x4000000000, 0), "hart init");
    status(k::tetoProcessInit(ctx, mem, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0), "process init");
    let at = reserve(ctx, mem, size);
    assert_eq!(v::tetoVfsReadData(ctx, mem, hello, 0, at, size as f64), size as f64);
    status(thx::tetoLoadThx(ctx, mem, 0.0, at, size as f64), "THX load");
    release(ctx, mem, at, size);
    print(b"REI: MIKUOS HELLO LOADED\n");

    // Smoke fixture only: the same zero entropy startup as the C target.
    let argv0 = b"/usr/bin/hello-rv64.39";
    let length = 36 + argv0.len();
    let at = reserve(ctx, mem, length);
    m::fill(ctx, mem, at, length as f64, 0.0);
    m::storeU32(ctx, mem, at, Numeric::Number(TETO_STARTUP_MAGIC as f64));
    m::storeU32(ctx, mem, at + 4.0, Numeric::Number(1.0));
    m::storeU32(ctx, mem, at + 32.0, Numeric::Number(argv0.len() as f64));
    copy(ctx, mem, at + 36.0, argv0);
    status(start::tetoBuildInitialStack(ctx, mem, 0.0, at, length as f64, 1048576.0), "startup");
    release(ctx, mem, at, length);
    let mut wrote = false;
    for _ in 0..1024 {
        poll();
        let result = k::tetoRunRv64Batch(ctx, mem, 0.0, 1024.0, micros(), 0.0);
        if result == TETO_BATCH_HOST as f64 {
            let op = k::tetoHostOperation(ctx, mem, 0.0);
            let fd = k::tetoHostDescriptor(ctx, mem, 0.0);
            let n = k::tetoHostLength(ctx, mem, 0.0);
            let address = k::tetoHostAddress(ctx, mem, 0.0);
            assert!(op == TETO_HOST_WRITE as f64 && (fd == 1.0 || fd == 2.0)
                && n >= 0.0 && n <= 65536.0 && n == n as u32 as f64, "host request");
            assert!(address >= 0 && address <= u64::MAX as i128 - n as i128);
            for i in 0..n as u32 {
                let byte = k::guestLoadU8(ctx, mem, 0.0, address + i as i128);
                assert!((0.0..=255.0).contains(&byte), "guest byte");
                unsafe { rei_console_char(byte as u8) };
            }
            wrote |= n > 0.0;
            status(k::tetoResumeHost(ctx, mem, 0.0, n as i128), "host resume");
        } else if result == TETO_BATCH_EXITED as f64 {
            status(k::tetoHartExitCode(ctx, mem, 0.0), "guest exit");
            assert!(wrote, "hello did not write");
            print(b"REI: MIKUOS HELLO EXIT PASS\n");
            return;
        } else {
            assert!(result == TETO_BATCH_BUDGET as f64, "guest stopped: {result}");
        }
    }
    panic!("guest instruction limit");
}

#[no_mangle]
pub extern "C" fn rei_native_start() -> ! {
    print(b"RUSTY-REI: RASPI5 AARCH64 BOOT\n");
    let raw = unsafe { rei_platform_allocate(POOL_SIZE) };
    assert!(!raw.is_null(), "arena allocation");
    // Circle owns this allocation until shutdown. Initialise before making a
    // Rust slice; no IRQ or second core receives a pointer to it.
    let bytes = unsafe {
        ptr::write_bytes(raw, 0, POOL_SIZE);
        slice::from_raw_parts_mut(raw, POOL_SIZE)
    };
    let ctx = Context::new(BoardMemory::new(bytes));
    let mem = m::directMemory(&ctx, (64 * 1024 * 1024) as f64, false);
    status(k::tetoKernelInit(&ctx, mem, 1.0, false), "kernel init");
    assert!(k::tetoKernelValid(&ctx, mem), "kernel validation");
    print(b"REI: CANONICAL KERNEL INIT PASS\n");
    let (at, size) = root_image(&ctx, mem);
    status(v::tetoLoadVfs(&ctx, mem, at, size as f64), "root mount");
    release(&ctx, mem, at, size);
    assert!(v::tetoVfsLoaded(&ctx, mem), "root not mounted");
    print(b"REI: MIKUOS ROOT MOUNT PASS\n");
    run_hello(&ctx, mem, v::tetoVfsRoot(&ctx, mem));
    unsafe { rei_console_idle() }
}
