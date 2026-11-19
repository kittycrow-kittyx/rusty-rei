#![no_std]
#![forbid(unsafe_code)]
#![allow(non_snake_case, non_camel_case_types, unused_variables, unused_mut, unused_imports, unused_parens, unused_labels)]
pub mod numbers;
pub use numbers::Numeric;
#[derive(Clone, Copy)] pub struct __bake_c_Buffer { pub handle: u64, pub byteLength: f64 }
#[derive(Clone, Copy)] pub struct __bake_c_View { pub buffer: __bake_c_Buffer }
#[derive(Clone, Copy)] pub struct __bake_c_ExternalMemory { pub buffer: __bake_c_Buffer }
#[derive(Clone, Copy)] pub struct __bake_c_U8Array { pub buffer: __bake_c_Buffer, pub length: f64 }
#[derive(Clone, Copy)] pub struct __bake_c_I32Array { pub buffer: __bake_c_Buffer, pub length: f64 }
#[derive(Clone, Copy)] pub struct __bake_c_U64Array { pub buffer: __bake_c_Buffer, pub length: f64 }
#[path = "core/native-types.rs"] pub mod m_core__native_types;
#[path = "native/types.rs"] pub mod m_native__types;
#[path = "native/abi.rs"] pub mod m_native__abi;
#[path = "core/rv64-branch.rs"] pub mod m_core__rv64_branch;
#[path = "core/rv64-numbers.rs"] pub mod m_core__rv64_numbers;
#[path = "vm/rv64-immediate.rs"] pub mod m_vm__rv64_immediate;
#[path = "core/rv64-arithmetic.rs"] pub mod m_core__rv64_arithmetic;
#[path = "core/rv64-word.rs"] pub mod m_core__rv64_word;
#[path = "core/rv64-immediate-op.rs"] pub mod m_core__rv64_immediate_op;
#[path = "core/rv64-register.rs"] pub mod m_core__rv64_register;
#[path = "core/rv64-fclass.rs"] pub mod m_core__rv64_fclass;
#[path = "native/memory.rs"] pub mod m_native__memory;
#[path = "native/word.rs"] pub mod m_native__word;
#[path = "native/vfs.rs"] pub mod m_native__vfs;
#[path = "native/kernel.rs"] pub mod m_native__kernel;
#[path = "native/start.rs"] pub mod m_native__start;
#[path = "native/thx.rs"] pub mod m_native__thx;

pub trait Platform {
    fn __bake_c_abort_range(&self) -> !;
    fn __bake_c_atomic_add_i32(&self, array: __bake_c_I32Array, at: f64, value: f64) -> f64;
    fn __bake_c_atomic_add_u64(&self, array: __bake_c_U64Array, at: f64, value: i128) -> i128;
    fn __bake_c_atomic_cmpxchg_i32(&self, array: __bake_c_I32Array, at: f64, expected: f64, replacement: f64) -> f64;
    fn __bake_c_atomic_load_i32(&self, array: __bake_c_I32Array, at: f64) -> f64;
    fn __bake_c_atomic_load_u64(&self, array: __bake_c_U64Array, at: f64) -> i128;
    fn __bake_c_atomic_store_i32(&self, array: __bake_c_I32Array, at: f64, value: f64) -> f64;
    fn __bake_c_atomic_store_u64(&self, array: __bake_c_U64Array, at: f64, value: i128) -> i128;
    fn __bake_c_i32_get(&self, array: __bake_c_I32Array, at: f64) -> f64;
    fn __bake_c_i32_set(&self, array: __bake_c_I32Array, at: f64, value: f64) -> f64;
    fn __bake_c_new_buffer(&self, size: f64) -> __bake_c_Buffer;
    fn __bake_c_new_i32_array(&self, buffer: __bake_c_Buffer) -> __bake_c_I32Array;
    fn __bake_c_new_shared_buffer(&self, size: f64) -> __bake_c_Buffer;
    fn __bake_c_new_u64_array(&self, buffer: __bake_c_Buffer) -> __bake_c_U64Array;
    fn __bake_c_new_u8_array(&self, buffer: __bake_c_Buffer) -> __bake_c_U8Array;
    fn __bake_c_new_view(&self, buffer: __bake_c_Buffer) -> __bake_c_View;
    fn __bake_c_u8_copy_within(&self, array: __bake_c_U8Array, target: f64, start: f64, end: Option<f64>) -> __bake_c_U8Array;
    fn __bake_c_u8_fill(&self, array: __bake_c_U8Array, value: f64, start: Option<f64>, end: Option<f64>) -> __bake_c_U8Array;
    fn __bake_c_u8_get(&self, array: __bake_c_U8Array, at: f64) -> f64;
    fn __bake_c_u8_set(&self, array: __bake_c_U8Array, at: f64, value: f64) -> f64;
    fn __bake_c_view_get_i16(&self, view: __bake_c_View, at: f64, littleEndian: Option<bool>) -> f64;
    fn __bake_c_view_get_i32(&self, view: __bake_c_View, at: f64, littleEndian: Option<bool>) -> f64;
    fn __bake_c_view_get_i64(&self, view: __bake_c_View, at: f64, littleEndian: Option<bool>) -> i128;
    fn __bake_c_view_get_u16(&self, view: __bake_c_View, at: f64, littleEndian: Option<bool>) -> f64;
    fn __bake_c_view_get_u32(&self, view: __bake_c_View, at: f64, littleEndian: Option<bool>) -> f64;
    fn __bake_c_view_get_u64(&self, view: __bake_c_View, at: f64, littleEndian: Option<bool>) -> i128;
    fn __bake_c_view_set_i32(&self, view: __bake_c_View, at: f64, value: f64, littleEndian: Option<bool>) -> ();
    fn __bake_c_view_set_i64(&self, view: __bake_c_View, at: f64, value: i128, littleEndian: Option<bool>) -> ();
    fn __bake_c_view_set_u16(&self, view: __bake_c_View, at: f64, value: f64, littleEndian: Option<bool>) -> ();
    fn __bake_c_view_set_u32(&self, view: __bake_c_View, at: f64, value: f64, littleEndian: Option<bool>) -> ();
    fn __bake_c_view_set_u64(&self, view: __bake_c_View, at: f64, value: i128, littleEndian: Option<bool>) -> ();
    fn __bake_c_f64_sqrt(&self, value: f64) -> f64;
    fn __bake_c_view_get_f32(&self, view: __bake_c_View, at: f64, littleEndian: Option<bool>) -> f64;
    fn __bake_c_view_get_f64(&self, view: __bake_c_View, at: f64, littleEndian: Option<bool>) -> f64;
    fn __bake_c_view_set_f32(&self, view: __bake_c_View, at: f64, value: f64, littleEndian: Option<bool>) -> ();
    fn __bake_c_view_set_f64(&self, view: __bake_c_View, at: f64, value: f64, littleEndian: Option<bool>) -> ();
}

pub struct Globals {
    pub(crate) m_native__abi__TETO_MAGIC: Option<f64>,
    pub(crate) m_native__abi__TETO_ABI_VERSION: Option<f64>,
    pub(crate) m_native__abi__TETO_CONTROL_BASE: Option<f64>,
    pub(crate) m_native__abi__TETO_CONTROL_SIZE: Option<f64>,
    pub(crate) m_native__abi__TETO_HART_BASE: Option<f64>,
    pub(crate) m_native__abi__TETO_HART_STRIDE: Option<f64>,
    pub(crate) m_native__abi__C_MAGIC: Option<f64>,
    pub(crate) m_native__abi__C_ABI: Option<f64>,
    pub(crate) m_native__abi__C_FLAGS: Option<f64>,
    pub(crate) m_native__abi__C_MAX_HARTS: Option<f64>,
    pub(crate) m_native__abi__C_ACTIVE_WORKERS: Option<f64>,
    pub(crate) m_native__abi__C_ATOMIC_LOCK: Option<f64>,
    pub(crate) m_native__abi__C_SCHED_LOCK: Option<f64>,
    pub(crate) m_native__abi__C_PROCESS_LOCK: Option<f64>,
    pub(crate) m_native__abi__C_VFS_LOCK: Option<f64>,
    pub(crate) m_native__abi__C_CREDENTIAL_LOCK: Option<f64>,
    pub(crate) m_native__abi__C_MEMORY_LOCK: Option<f64>,
    pub(crate) m_native__abi__C_PAGE_TABLE_BASE: Option<f64>,
    pub(crate) m_native__abi__C_PAGE_CAPACITY: Option<f64>,
    pub(crate) m_native__abi__C_PHYSICAL_NEXT: Option<f64>,
    pub(crate) m_native__abi__C_PHYSICAL_END: Option<f64>,
    pub(crate) m_native__abi__C_PROCESS_TABLE_BASE: Option<f64>,
    pub(crate) m_native__abi__C_PROCESS_CAPACITY: Option<f64>,
    pub(crate) m_native__abi__C_PROCESS_COUNT: Option<f64>,
    pub(crate) m_native__abi__C_NEXT_PID: Option<f64>,
    pub(crate) m_native__abi__C_SCHED_CURSOR: Option<f64>,
    pub(crate) m_native__abi__C_SCHED_RUNS: Option<f64>,
    pub(crate) m_native__abi__C_SCHED_IDLE: Option<f64>,
    pub(crate) m_native__abi__C_SCHED_CLAIMS: Option<f64>,
    pub(crate) m_native__abi__C_SCHED_CONTENTION: Option<f64>,
    pub(crate) m_native__abi__C_WORKER_CAPACITY: Option<f64>,
    pub(crate) m_native__abi__C_SEGMENT_TABLE_BASE: Option<f64>,
    pub(crate) m_native__abi__C_SEGMENT_CAPACITY: Option<f64>,
    pub(crate) m_native__abi__C_IMAGE_TOP: Option<f64>,
    pub(crate) m_native__abi__C_IMAGE_LOCK: Option<f64>,
    pub(crate) m_native__abi__C_IMAGE_SIZE: Option<f64>,
    pub(crate) m_native__abi__C_MAP_TABLE_BASE: Option<f64>,
    pub(crate) m_native__abi__C_MAP_CAPACITY: Option<f64>,
    pub(crate) m_native__abi__C_VFS_INODE_BASE: Option<f64>,
    pub(crate) m_native__abi__C_VFS_INODE_CAPACITY: Option<f64>,
    pub(crate) m_native__abi__C_VFS_DENTRY_BASE: Option<f64>,
    pub(crate) m_native__abi__C_VFS_DENTRY_CAPACITY: Option<f64>,
    pub(crate) m_native__abi__C_VFS_DATA_BASE: Option<f64>,
    pub(crate) m_native__abi__C_VFS_DATA_END: Option<f64>,
    pub(crate) m_native__abi__C_VFS_DATA_NEXT: Option<f64>,
    pub(crate) m_native__abi__C_VFS_INODE_COUNT: Option<f64>,
    pub(crate) m_native__abi__C_VFS_DENTRY_COUNT: Option<f64>,
    pub(crate) m_native__abi__C_VFS_ROOT: Option<f64>,
    pub(crate) m_native__abi__C_VFS_LOADED: Option<f64>,
    pub(crate) m_native__abi__C_VFS_DIRTY: Option<f64>,
    pub(crate) m_native__abi__C_VFS_GENERATION: Option<f64>,
    pub(crate) m_native__abi__C_VFS_SCRATCH_BASE: Option<f64>,
    pub(crate) m_native__abi__C_DESCRIPTOR_TABLE_BASE: Option<f64>,
    pub(crate) m_native__abi__C_DESCRIPTOR_CAPACITY: Option<f64>,
    pub(crate) m_native__abi__C_WORKER_BASE: Option<f64>,
    pub(crate) m_native__abi__TETO_WORKER_STRIDE: Option<f64>,
    pub(crate) m_native__abi__W_BATCHES: Option<f64>,
    pub(crate) m_native__abi__W_INSTRUCTIONS: Option<f64>,
    pub(crate) m_native__abi__W_IDLE: Option<f64>,
    pub(crate) m_native__abi__TETO_FLAG_THREADED: Option<f64>,
    pub(crate) m_native__abi__H_LOCK: Option<f64>,
    pub(crate) m_native__abi__H_STATUS: Option<f64>,
    pub(crate) m_native__abi__H_EVENT: Option<f64>,
    pub(crate) m_native__abi__H_FAULT: Option<f64>,
    pub(crate) m_native__abi__H_VIRTUAL_TOP: Option<f64>,
    pub(crate) m_native__abi__H_PC: Option<f64>,
    pub(crate) m_native__abi__H_RESERVATION: Option<f64>,
    pub(crate) m_native__abi__H_FCSR: Option<f64>,
    pub(crate) m_native__abi__H_LAST_INSTRUCTION: Option<f64>,
    pub(crate) m_native__abi__H_INSTRUCTIONS: Option<f64>,
    pub(crate) m_native__abi__H_HOST_TO_WASM: Option<f64>,
    pub(crate) m_native__abi__H_WASM_TO_HOST: Option<f64>,
    pub(crate) m_native__abi__H_SYNC_WAITS: Option<f64>,
    pub(crate) m_native__abi__H_LOCK_CONTENTION: Option<f64>,
    pub(crate) m_native__abi__H_MIGRATIONS: Option<f64>,
    pub(crate) m_native__abi__H_LAST_WORKER: Option<f64>,
    pub(crate) m_native__abi__H_PAGE_TABLE: Option<f64>,
    pub(crate) m_native__abi__H_PAGE_CAPACITY: Option<f64>,
    pub(crate) m_native__abi__H_PAGE_COUNT: Option<f64>,
    pub(crate) m_native__abi__H_X: Option<f64>,
    pub(crate) m_native__abi__H_F: Option<f64>,
    pub(crate) m_native__abi__H_HOST_OPERATION: Option<f64>,
    pub(crate) m_native__abi__H_HOST_DESCRIPTOR: Option<f64>,
    pub(crate) m_native__abi__H_HOST_ADDRESS: Option<f64>,
    pub(crate) m_native__abi__H_HOST_LENGTH: Option<f64>,
    pub(crate) m_native__abi__H_EXIT_CODE: Option<f64>,
    pub(crate) m_native__abi__H_PID: Option<f64>,
    pub(crate) m_native__abi__H_PPID: Option<f64>,
    pub(crate) m_native__abi__H_RUID: Option<f64>,
    pub(crate) m_native__abi__H_EUID: Option<f64>,
    pub(crate) m_native__abi__H_SUID: Option<f64>,
    pub(crate) m_native__abi__H_FSUID: Option<f64>,
    pub(crate) m_native__abi__H_RGID: Option<f64>,
    pub(crate) m_native__abi__H_EGID: Option<f64>,
    pub(crate) m_native__abi__H_SGID: Option<f64>,
    pub(crate) m_native__abi__H_FSGID: Option<f64>,
    pub(crate) m_native__abi__H_KERNEL_SYSCALLS: Option<f64>,
    pub(crate) m_native__abi__H_INTERNAL_SYSCALLS: Option<f64>,
    pub(crate) m_native__abi__H_HOST_REQUESTS: Option<f64>,
    pub(crate) m_native__abi__H_FALLBACK_SYSCALLS: Option<f64>,
    pub(crate) m_native__abi__H_GROUP_COUNT: Option<f64>,
    pub(crate) m_native__abi__H_GROUPS: Option<f64>,
    pub(crate) m_native__abi__TETO_GROUP_CAPACITY: Option<f64>,
    pub(crate) m_native__abi__H_PROCESS_SLOT: Option<f64>,
    pub(crate) m_native__abi__H_IMAGE_FLOOR: Option<f64>,
    pub(crate) m_native__abi__H_IMAGE_LOADS: Option<f64>,
    pub(crate) m_native__abi__H_IMAGE_BYTES: Option<f64>,
    pub(crate) m_native__abi__H_STACK_BOTTOM: Option<f64>,
    pub(crate) m_native__abi__H_STACK_POINTER: Option<f64>,
    pub(crate) m_native__abi__H_STARTUP_LOADS: Option<f64>,
    pub(crate) m_native__abi__H_FLOAT_SCRATCH: Option<f64>,
    pub(crate) m_native__abi__TETO_PROCESS_STRIDE: Option<f64>,
    pub(crate) m_native__abi__P_LOCK: Option<f64>,
    pub(crate) m_native__abi__P_STATE: Option<f64>,
    pub(crate) m_native__abi__P_PID: Option<f64>,
    pub(crate) m_native__abi__P_PPID: Option<f64>,
    pub(crate) m_native__abi__P_PGID: Option<f64>,
    pub(crate) m_native__abi__P_EXIT_CODE: Option<f64>,
    pub(crate) m_native__abi__P_RUID: Option<f64>,
    pub(crate) m_native__abi__P_EUID: Option<f64>,
    pub(crate) m_native__abi__P_SUID: Option<f64>,
    pub(crate) m_native__abi__P_FSUID: Option<f64>,
    pub(crate) m_native__abi__P_RGID: Option<f64>,
    pub(crate) m_native__abi__P_EGID: Option<f64>,
    pub(crate) m_native__abi__P_SGID: Option<f64>,
    pub(crate) m_native__abi__P_FSGID: Option<f64>,
    pub(crate) m_native__abi__P_GROUP_COUNT: Option<f64>,
    pub(crate) m_native__abi__P_GROUPS: Option<f64>,
    pub(crate) m_native__abi__P_PAGE_TABLE: Option<f64>,
    pub(crate) m_native__abi__P_PAGE_CAPACITY: Option<f64>,
    pub(crate) m_native__abi__P_PAGE_COUNT: Option<f64>,
    pub(crate) m_native__abi__P_VIRTUAL_TOP: Option<f64>,
    pub(crate) m_native__abi__P_SEGMENT_TABLE: Option<f64>,
    pub(crate) m_native__abi__P_SEGMENT_COUNT: Option<f64>,
    pub(crate) m_native__abi__P_IMAGE_FLOOR: Option<f64>,
    pub(crate) m_native__abi__P_IMAGE_ENTRY: Option<f64>,
    pub(crate) m_native__abi__P_PHDR: Option<f64>,
    pub(crate) m_native__abi__P_PHENT: Option<f64>,
    pub(crate) m_native__abi__P_PHNUM: Option<f64>,
    pub(crate) m_native__abi__P_STACK_BOTTOM: Option<f64>,
    pub(crate) m_native__abi__P_STACK_POINTER: Option<f64>,
    pub(crate) m_native__abi__P_MAP_TABLE: Option<f64>,
    pub(crate) m_native__abi__P_MAP_COUNT: Option<f64>,
    pub(crate) m_native__abi__P_MAP_CAPACITY: Option<f64>,
    pub(crate) m_native__abi__P_BRK_BASE: Option<f64>,
    pub(crate) m_native__abi__P_BRK: Option<f64>,
    pub(crate) m_native__abi__P_MAP_NEXT: Option<f64>,
    pub(crate) m_native__abi__P_CWD_INODE: Option<f64>,
    pub(crate) m_native__abi__P_UMASK: Option<f64>,
    pub(crate) m_native__abi__P_DESCRIPTOR_TABLE: Option<f64>,
    pub(crate) m_native__abi__P_DESCRIPTOR_CAPACITY: Option<f64>,
    pub(crate) m_native__abi__TETO_DESCRIPTOR_STRIDE: Option<f64>,
    pub(crate) m_native__abi__TETO_DESCRIPTOR_CAPACITY: Option<f64>,
    pub(crate) m_native__abi__FD_KIND: Option<f64>,
    pub(crate) m_native__abi__FD_FLAGS: Option<f64>,
    pub(crate) m_native__abi__FD_INODE: Option<f64>,
    pub(crate) m_native__abi__FD_OFFSET: Option<f64>,
    pub(crate) m_native__abi__FD_CURSOR: Option<f64>,
    pub(crate) m_native__abi__TETO_FD_EMPTY: Option<f64>,
    pub(crate) m_native__abi__TETO_FD_HOST: Option<f64>,
    pub(crate) m_native__abi__TETO_FD_FILE: Option<f64>,
    pub(crate) m_native__abi__TETO_FD_DIRECTORY: Option<f64>,
    pub(crate) m_native__abi__TETO_FD_READ: Option<f64>,
    pub(crate) m_native__abi__TETO_FD_WRITE: Option<f64>,
    pub(crate) m_native__abi__TETO_FD_APPEND: Option<f64>,
    pub(crate) m_native__abi__TETO_FD_CLOEXEC: Option<f64>,
    pub(crate) m_native__abi__TETO_VFS_PATH_CAPACITY: Option<f64>,
    pub(crate) m_native__abi__TETO_SEGMENT_STRIDE: Option<f64>,
    pub(crate) m_native__abi__TETO_SEGMENT_CAPACITY: Option<f64>,
    pub(crate) m_native__abi__S_ADDRESS: Option<f64>,
    pub(crate) m_native__abi__S_END: Option<f64>,
    pub(crate) m_native__abi__S_FLAGS: Option<f64>,
    pub(crate) m_native__abi__S_NAME_HASH: Option<f64>,
    pub(crate) m_native__abi__S_NAME_LENGTH: Option<f64>,
    pub(crate) m_native__abi__TETO_SEGMENT_READ: Option<f64>,
    pub(crate) m_native__abi__TETO_SEGMENT_WRITE: Option<f64>,
    pub(crate) m_native__abi__TETO_SEGMENT_EXECUTE: Option<f64>,
    pub(crate) m_native__abi__TETO_MAP_STRIDE: Option<f64>,
    pub(crate) m_native__abi__TETO_MAP_CAPACITY: Option<f64>,
    pub(crate) m_native__abi__M_ADDRESS: Option<f64>,
    pub(crate) m_native__abi__M_END: Option<f64>,
    pub(crate) m_native__abi__M_PROTECTION: Option<f64>,
    pub(crate) m_native__abi__M_FLAGS: Option<f64>,
    pub(crate) m_native__abi__M_FILE_DESCRIPTOR: Option<f64>,
    pub(crate) m_native__abi__M_FILE_OFFSET: Option<f64>,
    pub(crate) m_native__abi__M_BACKING: Option<f64>,
    pub(crate) m_native__abi__TETO_MAP_BACKING_ANONYMOUS: Option<f64>,
    pub(crate) m_native__abi__TETO_VFS_INODE_STRIDE: Option<f64>,
    pub(crate) m_native__abi__TETO_VFS_INODE_CAPACITY: Option<f64>,
    pub(crate) m_native__abi__TETO_VFS_DENTRY_STRIDE: Option<f64>,
    pub(crate) m_native__abi__TETO_VFS_DENTRY_CAPACITY: Option<f64>,
    pub(crate) m_native__abi__TETO_VFS_DATA_CAPACITY: Option<f64>,
    pub(crate) m_native__abi__V_KIND: Option<f64>,
    pub(crate) m_native__abi__V_MODE: Option<f64>,
    pub(crate) m_native__abi__V_UID: Option<f64>,
    pub(crate) m_native__abi__V_GID: Option<f64>,
    pub(crate) m_native__abi__V_NLINK: Option<f64>,
    pub(crate) m_native__abi__V_FLAGS: Option<f64>,
    pub(crate) m_native__abi__V_SIZE: Option<f64>,
    pub(crate) m_native__abi__V_DATA: Option<f64>,
    pub(crate) m_native__abi__V_CAPACITY: Option<f64>,
    pub(crate) m_native__abi__V_ATIME: Option<f64>,
    pub(crate) m_native__abi__V_MTIME: Option<f64>,
    pub(crate) m_native__abi__V_CTIME: Option<f64>,
    pub(crate) m_native__abi__D_PARENT: Option<f64>,
    pub(crate) m_native__abi__D_INODE: Option<f64>,
    pub(crate) m_native__abi__D_NAME: Option<f64>,
    pub(crate) m_native__abi__D_NAME_LENGTH: Option<f64>,
    pub(crate) m_native__abi__D_NAME_HASH: Option<f64>,
    pub(crate) m_native__abi__D_FLAGS: Option<f64>,
    pub(crate) m_native__abi__TETO_VFS_KIND_EMPTY: Option<f64>,
    pub(crate) m_native__abi__TETO_VFS_KIND_FILE: Option<f64>,
    pub(crate) m_native__abi__TETO_VFS_KIND_DIRECTORY: Option<f64>,
    pub(crate) m_native__abi__TETO_VFS_KIND_LINK: Option<f64>,
    pub(crate) m_native__abi__TETO_VFS_KIND_CHARACTER: Option<f64>,
    pub(crate) m_native__abi__TETO_VFS_IMAGE_MAGIC: Option<f64>,
    pub(crate) m_native__abi__TETO_VFS_IMAGE_VERSION: Option<f64>,
    pub(crate) m_native__abi__TETO_VFS_OK: Option<f64>,
    pub(crate) m_native__abi__TETO_VFS_RANGE: Option<f64>,
    pub(crate) m_native__abi__TETO_VFS_FORMAT: Option<f64>,
    pub(crate) m_native__abi__TETO_VFS_CHECKSUM: Option<f64>,
    pub(crate) m_native__abi__TETO_VFS_CAPACITY_ERROR: Option<f64>,
    pub(crate) m_native__abi__TETO_VFS_DUPLICATE: Option<f64>,
    pub(crate) m_native__abi__TETO_PARSER_BASE: Option<f64>,
    pub(crate) m_native__abi__TETO_PARSER_STRING: Option<f64>,
    pub(crate) m_native__abi__TETO_PARSER_STRING_LENGTH: Option<f64>,
    pub(crate) m_native__abi__TETO_PARSER_STRING_ESCAPED: Option<f64>,
    pub(crate) m_native__abi__TETO_PARSER_NUMBER: Option<f64>,
    pub(crate) m_native__abi__TETO_PROCESS_EMPTY: Option<f64>,
    pub(crate) m_native__abi__TETO_PROCESS_RUNNABLE: Option<f64>,
    pub(crate) m_native__abi__TETO_PROCESS_WAITING: Option<f64>,
    pub(crate) m_native__abi__TETO_PROCESS_ZOMBIE: Option<f64>,
    pub(crate) m_native__abi__TETO_GUEST_PAGE_SIZE: Option<f64>,
    pub(crate) m_native__abi__TETO_PAGE_ENTRY_SIZE: Option<f64>,
    pub(crate) m_native__abi__TETO_PAGE_CAPACITY: Option<f64>,
    pub(crate) m_native__abi__TETO_HART_EMPTY: Option<f64>,
    pub(crate) m_native__abi__TETO_HART_RUNNABLE: Option<f64>,
    pub(crate) m_native__abi__TETO_HART_WAITING: Option<f64>,
    pub(crate) m_native__abi__TETO_HART_EXITED: Option<f64>,
    pub(crate) m_native__abi__TETO_HART_FAULTED: Option<f64>,
    pub(crate) m_native__abi__TETO_EVENT_NONE: Option<f64>,
    pub(crate) m_native__abi__TETO_EVENT_SYSCALL: Option<f64>,
    pub(crate) m_native__abi__TETO_EVENT_HOST: Option<f64>,
    pub(crate) m_native__abi__TETO_HOST_NONE: Option<f64>,
    pub(crate) m_native__abi__TETO_HOST_WRITE: Option<f64>,
    pub(crate) m_native__abi__TETO_BATCH_BUDGET: Option<f64>,
    pub(crate) m_native__abi__TETO_BATCH_SYSCALL: Option<f64>,
    pub(crate) m_native__abi__TETO_BATCH_HOST: Option<f64>,
    pub(crate) m_native__abi__TETO_BATCH_EXITED: Option<f64>,
    pub(crate) m_native__abi__TETO_BATCH_FAULT: Option<f64>,
    pub(crate) m_native__abi__TETO_BATCH_BUSY: Option<f64>,
    pub(crate) m_native__abi__TETO_BATCH_CONTENDED: Option<f64>,
    pub(crate) m_native__abi__TETO_FAULT_NONE: Option<f64>,
    pub(crate) m_native__abi__TETO_FAULT_MEMORY: Option<f64>,
    pub(crate) m_native__abi__TETO_FAULT_INSTRUCTION: Option<f64>,
    pub(crate) m_native__abi__TETO_FAULT_BREAKPOINT: Option<f64>,
    pub(crate) m_native__abi__TETO_FAULT_BAD_STATE: Option<f64>,
    pub(crate) m_native__abi__TETO_THX_OK: Option<f64>,
    pub(crate) m_native__abi__TETO_THX_BUSY: Option<f64>,
    pub(crate) m_native__abi__TETO_THX_RANGE: Option<f64>,
    pub(crate) m_native__abi__TETO_THX_MAGIC: Option<f64>,
    pub(crate) m_native__abi__TETO_THX_LENGTH: Option<f64>,
    pub(crate) m_native__abi__TETO_THX_CHECKSUM: Option<f64>,
    pub(crate) m_native__abi__TETO_THX_JSON: Option<f64>,
    pub(crate) m_native__abi__TETO_THX_METADATA: Option<f64>,
    pub(crate) m_native__abi__TETO_THX_SECTION: Option<f64>,
    pub(crate) m_native__abi__TETO_THX_MEMORY: Option<f64>,
    pub(crate) m_native__abi__TETO_STARTUP_MAGIC: Option<f64>,
    pub(crate) m_native__abi__TETO_START_OK: Option<f64>,
    pub(crate) m_native__abi__TETO_START_RANGE: Option<f64>,
    pub(crate) m_native__abi__TETO_START_FORMAT: Option<f64>,
    pub(crate) m_native__abi__TETO_START_MEMORY: Option<f64>,
    pub(crate) m_core__rv64_arithmetic__MASK32: Option<i128>,
    pub(crate) m_native__word__MASK32: Option<i128>,
    pub(crate) m_native__vfs__VFS_HEADER_SIZE: Option<f64>,
    pub(crate) m_native__vfs__VFS_INODE_IMAGE_SIZE: Option<f64>,
    pub(crate) m_native__vfs__VFS_DENTRY_IMAGE_SIZE: Option<f64>,
    pub(crate) m_native__kernel__NO_RESERVATION: Option<i128>,
    pub(crate) m_native__kernel__INVALID_POINTER: Option<f64>,
    pub(crate) m_native__kernel__UNMAPPED_POINTER: Option<f64>,
    pub(crate) m_native__kernel__INVALID_PROCESS: Option<f64>,
    pub(crate) m_native__kernel__MAP_NOT_OWNED: Option<i128>,
    pub(crate) m_native__kernel__MAP_PAGE: Option<i128>,
    pub(crate) m_native__kernel__MAP_GUARD: Option<i128>,
    pub(crate) m_native__kernel__MAP_SPACING: Option<i128>,
    pub(crate) m_native__kernel__MMAP_TRACE_COUNTER: Option<f64>,
    pub(crate) m_native__kernel__MMAP_TRACE_BASE: Option<f64>,
    pub(crate) m_native__kernel__MMAP_TRACE_STRIDE: Option<f64>,
    pub(crate) m_native__kernel__MMAP_TRACE_CAPACITY: Option<f64>,
    pub(crate) m_native__kernel__MMAP_TRACE_START: Option<i128>,
    pub(crate) m_native__kernel__MMAP_TRACE_END: Option<i128>,
    pub(crate) m_native__kernel__OPEN_DIRECTORY: Option<f64>,
    pub(crate) m_native__kernel__OPEN_CLOEXEC: Option<f64>,
    pub(crate) m_native__start__START_BAD: Option<f64>,
    pub(crate) m_native__thx__BAD: Option<f64>,
    pub(crate) m_native__thx__MAX_SAFE: Option<i128>,
    pub(crate) m_native__thx__K_MACHINE: Option<f64>,
    pub(crate) m_native__thx__K_VER: Option<f64>,
    pub(crate) m_native__thx__K_SEC: Option<f64>,
    pub(crate) m_native__thx__K_SYM: Option<f64>,
    pub(crate) m_native__thx__K_REL: Option<f64>,
    pub(crate) m_native__thx__K_DBG: Option<f64>,
    pub(crate) m_native__thx__K_IDENT: Option<f64>,
    pub(crate) m_native__thx__K_ENTRY: Option<f64>,
    pub(crate) m_native__thx__K_MEM: Option<f64>,
    pub(crate) m_native__thx__K_ISA: Option<f64>,
    pub(crate) m_native__thx__K_PHDR: Option<f64>,
    pub(crate) m_native__thx__K_PHENT: Option<f64>,
    pub(crate) m_native__thx__K_PHNUM: Option<f64>,
    pub(crate) m_native__thx__K_NAME: Option<f64>,
    pub(crate) m_native__thx__K_FLG: Option<f64>,
    pub(crate) m_native__thx__K_ALIGN: Option<f64>,
    pub(crate) m_native__thx__K_SIZE: Option<f64>,
    pub(crate) m_native__thx__K_ADDR: Option<f64>,
    pub(crate) m_native__thx__K_AT: Option<f64>,
    pub(crate) m_native__thx__K_LEN: Option<f64>,
}

pub struct Context<P: Platform> { pub platform: P, globals: Globals }
impl<P: Platform> Context<P> {
    pub fn new(platform: P) -> Self {
        let mut ctx = Self { platform, globals: Globals {
            m_native__abi__TETO_MAGIC: None,
            m_native__abi__TETO_ABI_VERSION: None,
            m_native__abi__TETO_CONTROL_BASE: None,
            m_native__abi__TETO_CONTROL_SIZE: None,
            m_native__abi__TETO_HART_BASE: None,
            m_native__abi__TETO_HART_STRIDE: None,
            m_native__abi__C_MAGIC: None,
            m_native__abi__C_ABI: None,
            m_native__abi__C_FLAGS: None,
            m_native__abi__C_MAX_HARTS: None,
            m_native__abi__C_ACTIVE_WORKERS: None,
            m_native__abi__C_ATOMIC_LOCK: None,
            m_native__abi__C_SCHED_LOCK: None,
            m_native__abi__C_PROCESS_LOCK: None,
            m_native__abi__C_VFS_LOCK: None,
            m_native__abi__C_CREDENTIAL_LOCK: None,
            m_native__abi__C_MEMORY_LOCK: None,
            m_native__abi__C_PAGE_TABLE_BASE: None,
            m_native__abi__C_PAGE_CAPACITY: None,
            m_native__abi__C_PHYSICAL_NEXT: None,
            m_native__abi__C_PHYSICAL_END: None,
            m_native__abi__C_PROCESS_TABLE_BASE: None,
            m_native__abi__C_PROCESS_CAPACITY: None,
            m_native__abi__C_PROCESS_COUNT: None,
            m_native__abi__C_NEXT_PID: None,
            m_native__abi__C_SCHED_CURSOR: None,
            m_native__abi__C_SCHED_RUNS: None,
            m_native__abi__C_SCHED_IDLE: None,
            m_native__abi__C_SCHED_CLAIMS: None,
            m_native__abi__C_SCHED_CONTENTION: None,
            m_native__abi__C_WORKER_CAPACITY: None,
            m_native__abi__C_SEGMENT_TABLE_BASE: None,
            m_native__abi__C_SEGMENT_CAPACITY: None,
            m_native__abi__C_IMAGE_TOP: None,
            m_native__abi__C_IMAGE_LOCK: None,
            m_native__abi__C_IMAGE_SIZE: None,
            m_native__abi__C_MAP_TABLE_BASE: None,
            m_native__abi__C_MAP_CAPACITY: None,
            m_native__abi__C_VFS_INODE_BASE: None,
            m_native__abi__C_VFS_INODE_CAPACITY: None,
            m_native__abi__C_VFS_DENTRY_BASE: None,
            m_native__abi__C_VFS_DENTRY_CAPACITY: None,
            m_native__abi__C_VFS_DATA_BASE: None,
            m_native__abi__C_VFS_DATA_END: None,
            m_native__abi__C_VFS_DATA_NEXT: None,
            m_native__abi__C_VFS_INODE_COUNT: None,
            m_native__abi__C_VFS_DENTRY_COUNT: None,
            m_native__abi__C_VFS_ROOT: None,
            m_native__abi__C_VFS_LOADED: None,
            m_native__abi__C_VFS_DIRTY: None,
            m_native__abi__C_VFS_GENERATION: None,
            m_native__abi__C_VFS_SCRATCH_BASE: None,
            m_native__abi__C_DESCRIPTOR_TABLE_BASE: None,
            m_native__abi__C_DESCRIPTOR_CAPACITY: None,
            m_native__abi__C_WORKER_BASE: None,
            m_native__abi__TETO_WORKER_STRIDE: None,
            m_native__abi__W_BATCHES: None,
            m_native__abi__W_INSTRUCTIONS: None,
            m_native__abi__W_IDLE: None,
            m_native__abi__TETO_FLAG_THREADED: None,
            m_native__abi__H_LOCK: None,
            m_native__abi__H_STATUS: None,
            m_native__abi__H_EVENT: None,
            m_native__abi__H_FAULT: None,
            m_native__abi__H_VIRTUAL_TOP: None,
            m_native__abi__H_PC: None,
            m_native__abi__H_RESERVATION: None,
            m_native__abi__H_FCSR: None,
            m_native__abi__H_LAST_INSTRUCTION: None,
            m_native__abi__H_INSTRUCTIONS: None,
            m_native__abi__H_HOST_TO_WASM: None,
            m_native__abi__H_WASM_TO_HOST: None,
            m_native__abi__H_SYNC_WAITS: None,
            m_native__abi__H_LOCK_CONTENTION: None,
            m_native__abi__H_MIGRATIONS: None,
            m_native__abi__H_LAST_WORKER: None,
            m_native__abi__H_PAGE_TABLE: None,
            m_native__abi__H_PAGE_CAPACITY: None,
            m_native__abi__H_PAGE_COUNT: None,
            m_native__abi__H_X: None,
            m_native__abi__H_F: None,
            m_native__abi__H_HOST_OPERATION: None,
            m_native__abi__H_HOST_DESCRIPTOR: None,
            m_native__abi__H_HOST_ADDRESS: None,
            m_native__abi__H_HOST_LENGTH: None,
            m_native__abi__H_EXIT_CODE: None,
            m_native__abi__H_PID: None,
            m_native__abi__H_PPID: None,
            m_native__abi__H_RUID: None,
            m_native__abi__H_EUID: None,
            m_native__abi__H_SUID: None,
            m_native__abi__H_FSUID: None,
            m_native__abi__H_RGID: None,
            m_native__abi__H_EGID: None,
            m_native__abi__H_SGID: None,
            m_native__abi__H_FSGID: None,
            m_native__abi__H_KERNEL_SYSCALLS: None,
            m_native__abi__H_INTERNAL_SYSCALLS: None,
            m_native__abi__H_HOST_REQUESTS: None,
            m_native__abi__H_FALLBACK_SYSCALLS: None,
            m_native__abi__H_GROUP_COUNT: None,
            m_native__abi__H_GROUPS: None,
            m_native__abi__TETO_GROUP_CAPACITY: None,
            m_native__abi__H_PROCESS_SLOT: None,
            m_native__abi__H_IMAGE_FLOOR: None,
            m_native__abi__H_IMAGE_LOADS: None,
            m_native__abi__H_IMAGE_BYTES: None,
            m_native__abi__H_STACK_BOTTOM: None,
            m_native__abi__H_STACK_POINTER: None,
            m_native__abi__H_STARTUP_LOADS: None,
            m_native__abi__H_FLOAT_SCRATCH: None,
            m_native__abi__TETO_PROCESS_STRIDE: None,
            m_native__abi__P_LOCK: None,
            m_native__abi__P_STATE: None,
            m_native__abi__P_PID: None,
            m_native__abi__P_PPID: None,
            m_native__abi__P_PGID: None,
            m_native__abi__P_EXIT_CODE: None,
            m_native__abi__P_RUID: None,
            m_native__abi__P_EUID: None,
            m_native__abi__P_SUID: None,
            m_native__abi__P_FSUID: None,
            m_native__abi__P_RGID: None,
            m_native__abi__P_EGID: None,
            m_native__abi__P_SGID: None,
            m_native__abi__P_FSGID: None,
            m_native__abi__P_GROUP_COUNT: None,
            m_native__abi__P_GROUPS: None,
            m_native__abi__P_PAGE_TABLE: None,
            m_native__abi__P_PAGE_CAPACITY: None,
            m_native__abi__P_PAGE_COUNT: None,
            m_native__abi__P_VIRTUAL_TOP: None,
            m_native__abi__P_SEGMENT_TABLE: None,
            m_native__abi__P_SEGMENT_COUNT: None,
            m_native__abi__P_IMAGE_FLOOR: None,
            m_native__abi__P_IMAGE_ENTRY: None,
            m_native__abi__P_PHDR: None,
            m_native__abi__P_PHENT: None,
            m_native__abi__P_PHNUM: None,
            m_native__abi__P_STACK_BOTTOM: None,
            m_native__abi__P_STACK_POINTER: None,
            m_native__abi__P_MAP_TABLE: None,
            m_native__abi__P_MAP_COUNT: None,
            m_native__abi__P_MAP_CAPACITY: None,
            m_native__abi__P_BRK_BASE: None,
            m_native__abi__P_BRK: None,
            m_native__abi__P_MAP_NEXT: None,
            m_native__abi__P_CWD_INODE: None,
            m_native__abi__P_UMASK: None,
            m_native__abi__P_DESCRIPTOR_TABLE: None,
            m_native__abi__P_DESCRIPTOR_CAPACITY: None,
            m_native__abi__TETO_DESCRIPTOR_STRIDE: None,
            m_native__abi__TETO_DESCRIPTOR_CAPACITY: None,
            m_native__abi__FD_KIND: None,
            m_native__abi__FD_FLAGS: None,
            m_native__abi__FD_INODE: None,
            m_native__abi__FD_OFFSET: None,
            m_native__abi__FD_CURSOR: None,
            m_native__abi__TETO_FD_EMPTY: None,
            m_native__abi__TETO_FD_HOST: None,
            m_native__abi__TETO_FD_FILE: None,
            m_native__abi__TETO_FD_DIRECTORY: None,
            m_native__abi__TETO_FD_READ: None,
            m_native__abi__TETO_FD_WRITE: None,
            m_native__abi__TETO_FD_APPEND: None,
            m_native__abi__TETO_FD_CLOEXEC: None,
            m_native__abi__TETO_VFS_PATH_CAPACITY: None,
            m_native__abi__TETO_SEGMENT_STRIDE: None,
            m_native__abi__TETO_SEGMENT_CAPACITY: None,
            m_native__abi__S_ADDRESS: None,
            m_native__abi__S_END: None,
            m_native__abi__S_FLAGS: None,
            m_native__abi__S_NAME_HASH: None,
            m_native__abi__S_NAME_LENGTH: None,
            m_native__abi__TETO_SEGMENT_READ: None,
            m_native__abi__TETO_SEGMENT_WRITE: None,
            m_native__abi__TETO_SEGMENT_EXECUTE: None,
            m_native__abi__TETO_MAP_STRIDE: None,
            m_native__abi__TETO_MAP_CAPACITY: None,
            m_native__abi__M_ADDRESS: None,
            m_native__abi__M_END: None,
            m_native__abi__M_PROTECTION: None,
            m_native__abi__M_FLAGS: None,
            m_native__abi__M_FILE_DESCRIPTOR: None,
            m_native__abi__M_FILE_OFFSET: None,
            m_native__abi__M_BACKING: None,
            m_native__abi__TETO_MAP_BACKING_ANONYMOUS: None,
            m_native__abi__TETO_VFS_INODE_STRIDE: None,
            m_native__abi__TETO_VFS_INODE_CAPACITY: None,
            m_native__abi__TETO_VFS_DENTRY_STRIDE: None,
            m_native__abi__TETO_VFS_DENTRY_CAPACITY: None,
            m_native__abi__TETO_VFS_DATA_CAPACITY: None,
            m_native__abi__V_KIND: None,
            m_native__abi__V_MODE: None,
            m_native__abi__V_UID: None,
            m_native__abi__V_GID: None,
            m_native__abi__V_NLINK: None,
            m_native__abi__V_FLAGS: None,
            m_native__abi__V_SIZE: None,
            m_native__abi__V_DATA: None,
            m_native__abi__V_CAPACITY: None,
            m_native__abi__V_ATIME: None,
            m_native__abi__V_MTIME: None,
            m_native__abi__V_CTIME: None,
            m_native__abi__D_PARENT: None,
            m_native__abi__D_INODE: None,
            m_native__abi__D_NAME: None,
            m_native__abi__D_NAME_LENGTH: None,
            m_native__abi__D_NAME_HASH: None,
            m_native__abi__D_FLAGS: None,
            m_native__abi__TETO_VFS_KIND_EMPTY: None,
            m_native__abi__TETO_VFS_KIND_FILE: None,
            m_native__abi__TETO_VFS_KIND_DIRECTORY: None,
            m_native__abi__TETO_VFS_KIND_LINK: None,
            m_native__abi__TETO_VFS_KIND_CHARACTER: None,
            m_native__abi__TETO_VFS_IMAGE_MAGIC: None,
            m_native__abi__TETO_VFS_IMAGE_VERSION: None,
            m_native__abi__TETO_VFS_OK: None,
            m_native__abi__TETO_VFS_RANGE: None,
            m_native__abi__TETO_VFS_FORMAT: None,
            m_native__abi__TETO_VFS_CHECKSUM: None,
            m_native__abi__TETO_VFS_CAPACITY_ERROR: None,
            m_native__abi__TETO_VFS_DUPLICATE: None,
            m_native__abi__TETO_PARSER_BASE: None,
            m_native__abi__TETO_PARSER_STRING: None,
            m_native__abi__TETO_PARSER_STRING_LENGTH: None,
            m_native__abi__TETO_PARSER_STRING_ESCAPED: None,
            m_native__abi__TETO_PARSER_NUMBER: None,
            m_native__abi__TETO_PROCESS_EMPTY: None,
            m_native__abi__TETO_PROCESS_RUNNABLE: None,
            m_native__abi__TETO_PROCESS_WAITING: None,
            m_native__abi__TETO_PROCESS_ZOMBIE: None,
            m_native__abi__TETO_GUEST_PAGE_SIZE: None,
            m_native__abi__TETO_PAGE_ENTRY_SIZE: None,
            m_native__abi__TETO_PAGE_CAPACITY: None,
            m_native__abi__TETO_HART_EMPTY: None,
            m_native__abi__TETO_HART_RUNNABLE: None,
            m_native__abi__TETO_HART_WAITING: None,
            m_native__abi__TETO_HART_EXITED: None,
            m_native__abi__TETO_HART_FAULTED: None,
            m_native__abi__TETO_EVENT_NONE: None,
            m_native__abi__TETO_EVENT_SYSCALL: None,
            m_native__abi__TETO_EVENT_HOST: None,
            m_native__abi__TETO_HOST_NONE: None,
            m_native__abi__TETO_HOST_WRITE: None,
            m_native__abi__TETO_BATCH_BUDGET: None,
            m_native__abi__TETO_BATCH_SYSCALL: None,
            m_native__abi__TETO_BATCH_HOST: None,
            m_native__abi__TETO_BATCH_EXITED: None,
            m_native__abi__TETO_BATCH_FAULT: None,
            m_native__abi__TETO_BATCH_BUSY: None,
            m_native__abi__TETO_BATCH_CONTENDED: None,
            m_native__abi__TETO_FAULT_NONE: None,
            m_native__abi__TETO_FAULT_MEMORY: None,
            m_native__abi__TETO_FAULT_INSTRUCTION: None,
            m_native__abi__TETO_FAULT_BREAKPOINT: None,
            m_native__abi__TETO_FAULT_BAD_STATE: None,
            m_native__abi__TETO_THX_OK: None,
            m_native__abi__TETO_THX_BUSY: None,
            m_native__abi__TETO_THX_RANGE: None,
            m_native__abi__TETO_THX_MAGIC: None,
            m_native__abi__TETO_THX_LENGTH: None,
            m_native__abi__TETO_THX_CHECKSUM: None,
            m_native__abi__TETO_THX_JSON: None,
            m_native__abi__TETO_THX_METADATA: None,
            m_native__abi__TETO_THX_SECTION: None,
            m_native__abi__TETO_THX_MEMORY: None,
            m_native__abi__TETO_STARTUP_MAGIC: None,
            m_native__abi__TETO_START_OK: None,
            m_native__abi__TETO_START_RANGE: None,
            m_native__abi__TETO_START_FORMAT: None,
            m_native__abi__TETO_START_MEMORY: None,
            m_core__rv64_arithmetic__MASK32: None,
            m_native__word__MASK32: None,
            m_native__vfs__VFS_HEADER_SIZE: None,
            m_native__vfs__VFS_INODE_IMAGE_SIZE: None,
            m_native__vfs__VFS_DENTRY_IMAGE_SIZE: None,
            m_native__kernel__NO_RESERVATION: None,
            m_native__kernel__INVALID_POINTER: None,
            m_native__kernel__UNMAPPED_POINTER: None,
            m_native__kernel__INVALID_PROCESS: None,
            m_native__kernel__MAP_NOT_OWNED: None,
            m_native__kernel__MAP_PAGE: None,
            m_native__kernel__MAP_GUARD: None,
            m_native__kernel__MAP_SPACING: None,
            m_native__kernel__MMAP_TRACE_COUNTER: None,
            m_native__kernel__MMAP_TRACE_BASE: None,
            m_native__kernel__MMAP_TRACE_STRIDE: None,
            m_native__kernel__MMAP_TRACE_CAPACITY: None,
            m_native__kernel__MMAP_TRACE_START: None,
            m_native__kernel__MMAP_TRACE_END: None,
            m_native__kernel__OPEN_DIRECTORY: None,
            m_native__kernel__OPEN_CLOEXEC: None,
            m_native__start__START_BAD: None,
            m_native__thx__BAD: None,
            m_native__thx__MAX_SAFE: None,
            m_native__thx__K_MACHINE: None,
            m_native__thx__K_VER: None,
            m_native__thx__K_SEC: None,
            m_native__thx__K_SYM: None,
            m_native__thx__K_REL: None,
            m_native__thx__K_DBG: None,
            m_native__thx__K_IDENT: None,
            m_native__thx__K_ENTRY: None,
            m_native__thx__K_MEM: None,
            m_native__thx__K_ISA: None,
            m_native__thx__K_PHDR: None,
            m_native__thx__K_PHENT: None,
            m_native__thx__K_PHNUM: None,
            m_native__thx__K_NAME: None,
            m_native__thx__K_FLG: None,
            m_native__thx__K_ALIGN: None,
            m_native__thx__K_SIZE: None,
            m_native__thx__K_ADDR: None,
            m_native__thx__K_AT: None,
            m_native__thx__K_LEN: None,
        } };
        ctx.globals.m_native__abi__TETO_MAGIC = Some(1330922836.0f64);
        ctx.globals.m_native__abi__TETO_ABI_VERSION = Some(10.0f64);
        ctx.globals.m_native__abi__TETO_CONTROL_BASE = Some(0.0f64);
        ctx.globals.m_native__abi__TETO_CONTROL_SIZE = Some(4096.0f64);
        ctx.globals.m_native__abi__TETO_HART_BASE = Some(4096.0f64);
        ctx.globals.m_native__abi__TETO_HART_STRIDE = Some(1024.0f64);
        ctx.globals.m_native__abi__C_MAGIC = Some(0.0f64);
        ctx.globals.m_native__abi__C_ABI = Some(4.0f64);
        ctx.globals.m_native__abi__C_FLAGS = Some(8.0f64);
        ctx.globals.m_native__abi__C_MAX_HARTS = Some(12.0f64);
        ctx.globals.m_native__abi__C_ACTIVE_WORKERS = Some(16.0f64);
        ctx.globals.m_native__abi__C_ATOMIC_LOCK = Some(20.0f64);
        ctx.globals.m_native__abi__C_SCHED_LOCK = Some(24.0f64);
        ctx.globals.m_native__abi__C_PROCESS_LOCK = Some(28.0f64);
        ctx.globals.m_native__abi__C_VFS_LOCK = Some(32.0f64);
        ctx.globals.m_native__abi__C_CREDENTIAL_LOCK = Some(36.0f64);
        ctx.globals.m_native__abi__C_MEMORY_LOCK = Some(40.0f64);
        ctx.globals.m_native__abi__C_PAGE_TABLE_BASE = Some(44.0f64);
        ctx.globals.m_native__abi__C_PAGE_CAPACITY = Some(48.0f64);
        ctx.globals.m_native__abi__C_PHYSICAL_NEXT = Some(52.0f64);
        ctx.globals.m_native__abi__C_PHYSICAL_END = Some(56.0f64);
        ctx.globals.m_native__abi__C_PROCESS_TABLE_BASE = Some(60.0f64);
        ctx.globals.m_native__abi__C_PROCESS_CAPACITY = Some(64.0f64);
        ctx.globals.m_native__abi__C_PROCESS_COUNT = Some(68.0f64);
        ctx.globals.m_native__abi__C_NEXT_PID = Some(72.0f64);
        ctx.globals.m_native__abi__C_SCHED_CURSOR = Some(76.0f64);
        ctx.globals.m_native__abi__C_SCHED_RUNS = Some(80.0f64);
        ctx.globals.m_native__abi__C_SCHED_IDLE = Some(88.0f64);
        ctx.globals.m_native__abi__C_SCHED_CLAIMS = Some(96.0f64);
        ctx.globals.m_native__abi__C_SCHED_CONTENTION = Some(104.0f64);
        ctx.globals.m_native__abi__C_WORKER_CAPACITY = Some(112.0f64);
        ctx.globals.m_native__abi__C_SEGMENT_TABLE_BASE = Some(116.0f64);
        ctx.globals.m_native__abi__C_SEGMENT_CAPACITY = Some(120.0f64);
        ctx.globals.m_native__abi__C_IMAGE_TOP = Some(124.0f64);
        ctx.globals.m_native__abi__C_IMAGE_LOCK = Some(128.0f64);
        ctx.globals.m_native__abi__C_IMAGE_SIZE = Some(132.0f64);
        ctx.globals.m_native__abi__C_MAP_TABLE_BASE = Some(136.0f64);
        ctx.globals.m_native__abi__C_MAP_CAPACITY = Some(140.0f64);
        ctx.globals.m_native__abi__C_VFS_INODE_BASE = Some(3232.0f64);
        ctx.globals.m_native__abi__C_VFS_INODE_CAPACITY = Some(3236.0f64);
        ctx.globals.m_native__abi__C_VFS_DENTRY_BASE = Some(3240.0f64);
        ctx.globals.m_native__abi__C_VFS_DENTRY_CAPACITY = Some(3244.0f64);
        ctx.globals.m_native__abi__C_VFS_DATA_BASE = Some(3248.0f64);
        ctx.globals.m_native__abi__C_VFS_DATA_END = Some(3252.0f64);
        ctx.globals.m_native__abi__C_VFS_DATA_NEXT = Some(3256.0f64);
        ctx.globals.m_native__abi__C_VFS_INODE_COUNT = Some(3260.0f64);
        ctx.globals.m_native__abi__C_VFS_DENTRY_COUNT = Some(3264.0f64);
        ctx.globals.m_native__abi__C_VFS_ROOT = Some(3268.0f64);
        ctx.globals.m_native__abi__C_VFS_LOADED = Some(3272.0f64);
        ctx.globals.m_native__abi__C_VFS_DIRTY = Some(3276.0f64);
        ctx.globals.m_native__abi__C_VFS_GENERATION = Some(3280.0f64);
        ctx.globals.m_native__abi__C_VFS_SCRATCH_BASE = Some(3288.0f64);
        ctx.globals.m_native__abi__C_DESCRIPTOR_TABLE_BASE = Some(3292.0f64);
        ctx.globals.m_native__abi__C_DESCRIPTOR_CAPACITY = Some(3296.0f64);
        ctx.globals.m_native__abi__C_WORKER_BASE = Some(160.0f64);
        ctx.globals.m_native__abi__TETO_WORKER_STRIDE = Some(24.0f64);
        ctx.globals.m_native__abi__W_BATCHES = Some(0.0f64);
        ctx.globals.m_native__abi__W_INSTRUCTIONS = Some(8.0f64);
        ctx.globals.m_native__abi__W_IDLE = Some(16.0f64);
        ctx.globals.m_native__abi__TETO_FLAG_THREADED = Some(1.0f64);
        ctx.globals.m_native__abi__H_LOCK = Some(0.0f64);
        ctx.globals.m_native__abi__H_STATUS = Some(4.0f64);
        ctx.globals.m_native__abi__H_EVENT = Some(8.0f64);
        ctx.globals.m_native__abi__H_FAULT = Some(12.0f64);
        ctx.globals.m_native__abi__H_VIRTUAL_TOP = Some(16.0f64);
        ctx.globals.m_native__abi__H_PC = Some(24.0f64);
        ctx.globals.m_native__abi__H_RESERVATION = Some(32.0f64);
        ctx.globals.m_native__abi__H_FCSR = Some(40.0f64);
        ctx.globals.m_native__abi__H_LAST_INSTRUCTION = Some(44.0f64);
        ctx.globals.m_native__abi__H_INSTRUCTIONS = Some(48.0f64);
        ctx.globals.m_native__abi__H_HOST_TO_WASM = Some(56.0f64);
        ctx.globals.m_native__abi__H_WASM_TO_HOST = Some(64.0f64);
        ctx.globals.m_native__abi__H_SYNC_WAITS = Some(72.0f64);
        ctx.globals.m_native__abi__H_LOCK_CONTENTION = Some(80.0f64);
        ctx.globals.m_native__abi__H_MIGRATIONS = Some(88.0f64);
        ctx.globals.m_native__abi__H_LAST_WORKER = Some(96.0f64);
        ctx.globals.m_native__abi__H_PAGE_TABLE = Some(104.0f64);
        ctx.globals.m_native__abi__H_PAGE_CAPACITY = Some(108.0f64);
        ctx.globals.m_native__abi__H_PAGE_COUNT = Some(112.0f64);
        ctx.globals.m_native__abi__H_X = Some(128.0f64);
        ctx.globals.m_native__abi__H_F = Some(384.0f64);
        ctx.globals.m_native__abi__H_HOST_OPERATION = Some(640.0f64);
        ctx.globals.m_native__abi__H_HOST_DESCRIPTOR = Some(644.0f64);
        ctx.globals.m_native__abi__H_HOST_ADDRESS = Some(648.0f64);
        ctx.globals.m_native__abi__H_HOST_LENGTH = Some(656.0f64);
        ctx.globals.m_native__abi__H_EXIT_CODE = Some(660.0f64);
        ctx.globals.m_native__abi__H_PID = Some(664.0f64);
        ctx.globals.m_native__abi__H_PPID = Some(668.0f64);
        ctx.globals.m_native__abi__H_RUID = Some(672.0f64);
        ctx.globals.m_native__abi__H_EUID = Some(676.0f64);
        ctx.globals.m_native__abi__H_SUID = Some(680.0f64);
        ctx.globals.m_native__abi__H_FSUID = Some(684.0f64);
        ctx.globals.m_native__abi__H_RGID = Some(688.0f64);
        ctx.globals.m_native__abi__H_EGID = Some(692.0f64);
        ctx.globals.m_native__abi__H_SGID = Some(696.0f64);
        ctx.globals.m_native__abi__H_FSGID = Some(700.0f64);
        ctx.globals.m_native__abi__H_KERNEL_SYSCALLS = Some(704.0f64);
        ctx.globals.m_native__abi__H_INTERNAL_SYSCALLS = Some(712.0f64);
        ctx.globals.m_native__abi__H_HOST_REQUESTS = Some(720.0f64);
        ctx.globals.m_native__abi__H_FALLBACK_SYSCALLS = Some(728.0f64);
        ctx.globals.m_native__abi__H_GROUP_COUNT = Some(736.0f64);
        ctx.globals.m_native__abi__H_GROUPS = Some(740.0f64);
        ctx.globals.m_native__abi__TETO_GROUP_CAPACITY = Some(32.0f64);
        ctx.globals.m_native__abi__H_PROCESS_SLOT = Some(868.0f64);
        ctx.globals.m_native__abi__H_IMAGE_FLOOR = Some(872.0f64);
        ctx.globals.m_native__abi__H_IMAGE_LOADS = Some(880.0f64);
        ctx.globals.m_native__abi__H_IMAGE_BYTES = Some(888.0f64);
        ctx.globals.m_native__abi__H_STACK_BOTTOM = Some(896.0f64);
        ctx.globals.m_native__abi__H_STACK_POINTER = Some(904.0f64);
        ctx.globals.m_native__abi__H_STARTUP_LOADS = Some(912.0f64);
        ctx.globals.m_native__abi__H_FLOAT_SCRATCH = Some(920.0f64);
        ctx.globals.m_native__abi__TETO_PROCESS_STRIDE = Some(352.0f64);
        ctx.globals.m_native__abi__P_LOCK = Some(0.0f64);
        ctx.globals.m_native__abi__P_STATE = Some(4.0f64);
        ctx.globals.m_native__abi__P_PID = Some(8.0f64);
        ctx.globals.m_native__abi__P_PPID = Some(12.0f64);
        ctx.globals.m_native__abi__P_PGID = Some(16.0f64);
        ctx.globals.m_native__abi__P_EXIT_CODE = Some(20.0f64);
        ctx.globals.m_native__abi__P_RUID = Some(24.0f64);
        ctx.globals.m_native__abi__P_EUID = Some(28.0f64);
        ctx.globals.m_native__abi__P_SUID = Some(32.0f64);
        ctx.globals.m_native__abi__P_FSUID = Some(36.0f64);
        ctx.globals.m_native__abi__P_RGID = Some(40.0f64);
        ctx.globals.m_native__abi__P_EGID = Some(44.0f64);
        ctx.globals.m_native__abi__P_SGID = Some(48.0f64);
        ctx.globals.m_native__abi__P_FSGID = Some(52.0f64);
        ctx.globals.m_native__abi__P_GROUP_COUNT = Some(56.0f64);
        ctx.globals.m_native__abi__P_GROUPS = Some(64.0f64);
        ctx.globals.m_native__abi__P_PAGE_TABLE = Some(192.0f64);
        ctx.globals.m_native__abi__P_PAGE_CAPACITY = Some(196.0f64);
        ctx.globals.m_native__abi__P_PAGE_COUNT = Some(200.0f64);
        ctx.globals.m_native__abi__P_VIRTUAL_TOP = Some(208.0f64);
        ctx.globals.m_native__abi__P_SEGMENT_TABLE = Some(216.0f64);
        ctx.globals.m_native__abi__P_SEGMENT_COUNT = Some(220.0f64);
        ctx.globals.m_native__abi__P_IMAGE_FLOOR = Some(224.0f64);
        ctx.globals.m_native__abi__P_IMAGE_ENTRY = Some(232.0f64);
        ctx.globals.m_native__abi__P_PHDR = Some(240.0f64);
        ctx.globals.m_native__abi__P_PHENT = Some(248.0f64);
        ctx.globals.m_native__abi__P_PHNUM = Some(252.0f64);
        ctx.globals.m_native__abi__P_STACK_BOTTOM = Some(256.0f64);
        ctx.globals.m_native__abi__P_STACK_POINTER = Some(264.0f64);
        ctx.globals.m_native__abi__P_MAP_TABLE = Some(272.0f64);
        ctx.globals.m_native__abi__P_MAP_COUNT = Some(276.0f64);
        ctx.globals.m_native__abi__P_MAP_CAPACITY = Some(280.0f64);
        ctx.globals.m_native__abi__P_BRK_BASE = Some(288.0f64);
        ctx.globals.m_native__abi__P_BRK = Some(296.0f64);
        ctx.globals.m_native__abi__P_MAP_NEXT = Some(304.0f64);
        ctx.globals.m_native__abi__P_CWD_INODE = Some(312.0f64);
        ctx.globals.m_native__abi__P_UMASK = Some(316.0f64);
        ctx.globals.m_native__abi__P_DESCRIPTOR_TABLE = Some(320.0f64);
        ctx.globals.m_native__abi__P_DESCRIPTOR_CAPACITY = Some(324.0f64);
        ctx.globals.m_native__abi__TETO_DESCRIPTOR_STRIDE = Some(32.0f64);
        ctx.globals.m_native__abi__TETO_DESCRIPTOR_CAPACITY = Some(64.0f64);
        ctx.globals.m_native__abi__FD_KIND = Some(0.0f64);
        ctx.globals.m_native__abi__FD_FLAGS = Some(4.0f64);
        ctx.globals.m_native__abi__FD_INODE = Some(8.0f64);
        ctx.globals.m_native__abi__FD_OFFSET = Some(16.0f64);
        ctx.globals.m_native__abi__FD_CURSOR = Some(24.0f64);
        ctx.globals.m_native__abi__TETO_FD_EMPTY = Some(0.0f64);
        ctx.globals.m_native__abi__TETO_FD_HOST = Some(1.0f64);
        ctx.globals.m_native__abi__TETO_FD_FILE = Some(2.0f64);
        ctx.globals.m_native__abi__TETO_FD_DIRECTORY = Some(3.0f64);
        ctx.globals.m_native__abi__TETO_FD_READ = Some(1.0f64);
        ctx.globals.m_native__abi__TETO_FD_WRITE = Some(2.0f64);
        ctx.globals.m_native__abi__TETO_FD_APPEND = Some(4.0f64);
        ctx.globals.m_native__abi__TETO_FD_CLOEXEC = Some(8.0f64);
        ctx.globals.m_native__abi__TETO_VFS_PATH_CAPACITY = Some(4096.0f64);
        ctx.globals.m_native__abi__TETO_SEGMENT_STRIDE = Some(32.0f64);
        ctx.globals.m_native__abi__TETO_SEGMENT_CAPACITY = Some(128.0f64);
        ctx.globals.m_native__abi__S_ADDRESS = Some(0.0f64);
        ctx.globals.m_native__abi__S_END = Some(8.0f64);
        ctx.globals.m_native__abi__S_FLAGS = Some(16.0f64);
        ctx.globals.m_native__abi__S_NAME_HASH = Some(20.0f64);
        ctx.globals.m_native__abi__S_NAME_LENGTH = Some(24.0f64);
        ctx.globals.m_native__abi__TETO_SEGMENT_READ = Some(1.0f64);
        ctx.globals.m_native__abi__TETO_SEGMENT_WRITE = Some(2.0f64);
        ctx.globals.m_native__abi__TETO_SEGMENT_EXECUTE = Some(4.0f64);
        ctx.globals.m_native__abi__TETO_MAP_STRIDE = Some(48.0f64);
        ctx.globals.m_native__abi__TETO_MAP_CAPACITY = Some(256.0f64);
        ctx.globals.m_native__abi__M_ADDRESS = Some(0.0f64);
        ctx.globals.m_native__abi__M_END = Some(8.0f64);
        ctx.globals.m_native__abi__M_PROTECTION = Some(16.0f64);
        ctx.globals.m_native__abi__M_FLAGS = Some(20.0f64);
        ctx.globals.m_native__abi__M_FILE_DESCRIPTOR = Some(24.0f64);
        ctx.globals.m_native__abi__M_FILE_OFFSET = Some(32.0f64);
        ctx.globals.m_native__abi__M_BACKING = Some(40.0f64);
        ctx.globals.m_native__abi__TETO_MAP_BACKING_ANONYMOUS = Some(0.0f64);
        ctx.globals.m_native__abi__TETO_VFS_INODE_STRIDE = Some(64.0f64);
        ctx.globals.m_native__abi__TETO_VFS_INODE_CAPACITY = Some(4096.0f64);
        ctx.globals.m_native__abi__TETO_VFS_DENTRY_STRIDE = Some(24.0f64);
        ctx.globals.m_native__abi__TETO_VFS_DENTRY_CAPACITY = Some(8192.0f64);
        ctx.globals.m_native__abi__TETO_VFS_DATA_CAPACITY = Some(((16.0f64 * 1024.0f64) * 1024.0f64));
        ctx.globals.m_native__abi__V_KIND = Some(0.0f64);
        ctx.globals.m_native__abi__V_MODE = Some(4.0f64);
        ctx.globals.m_native__abi__V_UID = Some(8.0f64);
        ctx.globals.m_native__abi__V_GID = Some(12.0f64);
        ctx.globals.m_native__abi__V_NLINK = Some(16.0f64);
        ctx.globals.m_native__abi__V_FLAGS = Some(20.0f64);
        ctx.globals.m_native__abi__V_SIZE = Some(24.0f64);
        ctx.globals.m_native__abi__V_DATA = Some(32.0f64);
        ctx.globals.m_native__abi__V_CAPACITY = Some(36.0f64);
        ctx.globals.m_native__abi__V_ATIME = Some(40.0f64);
        ctx.globals.m_native__abi__V_MTIME = Some(48.0f64);
        ctx.globals.m_native__abi__V_CTIME = Some(56.0f64);
        ctx.globals.m_native__abi__D_PARENT = Some(0.0f64);
        ctx.globals.m_native__abi__D_INODE = Some(4.0f64);
        ctx.globals.m_native__abi__D_NAME = Some(8.0f64);
        ctx.globals.m_native__abi__D_NAME_LENGTH = Some(12.0f64);
        ctx.globals.m_native__abi__D_NAME_HASH = Some(16.0f64);
        ctx.globals.m_native__abi__D_FLAGS = Some(20.0f64);
        ctx.globals.m_native__abi__TETO_VFS_KIND_EMPTY = Some(0.0f64);
        ctx.globals.m_native__abi__TETO_VFS_KIND_FILE = Some(1.0f64);
        ctx.globals.m_native__abi__TETO_VFS_KIND_DIRECTORY = Some(2.0f64);
        ctx.globals.m_native__abi__TETO_VFS_KIND_LINK = Some(3.0f64);
        ctx.globals.m_native__abi__TETO_VFS_KIND_CHARACTER = Some(4.0f64);
        ctx.globals.m_native__abi__TETO_VFS_IMAGE_MAGIC = Some(827541078.0f64);
        ctx.globals.m_native__abi__TETO_VFS_IMAGE_VERSION = Some(1.0f64);
        ctx.globals.m_native__abi__TETO_VFS_OK = Some(0.0f64);
        ctx.globals.m_native__abi__TETO_VFS_RANGE = Some(1.0f64);
        ctx.globals.m_native__abi__TETO_VFS_FORMAT = Some(2.0f64);
        ctx.globals.m_native__abi__TETO_VFS_CHECKSUM = Some(3.0f64);
        ctx.globals.m_native__abi__TETO_VFS_CAPACITY_ERROR = Some(4.0f64);
        ctx.globals.m_native__abi__TETO_VFS_DUPLICATE = Some(5.0f64);
        ctx.globals.m_native__abi__TETO_PARSER_BASE = Some(3328.0f64);
        ctx.globals.m_native__abi__TETO_PARSER_STRING = Some(0.0f64);
        ctx.globals.m_native__abi__TETO_PARSER_STRING_LENGTH = Some(4.0f64);
        ctx.globals.m_native__abi__TETO_PARSER_STRING_ESCAPED = Some(8.0f64);
        ctx.globals.m_native__abi__TETO_PARSER_NUMBER = Some(16.0f64);
        ctx.globals.m_native__abi__TETO_PROCESS_EMPTY = Some(0.0f64);
        ctx.globals.m_native__abi__TETO_PROCESS_RUNNABLE = Some(1.0f64);
        ctx.globals.m_native__abi__TETO_PROCESS_WAITING = Some(2.0f64);
        ctx.globals.m_native__abi__TETO_PROCESS_ZOMBIE = Some(3.0f64);
        ctx.globals.m_native__abi__TETO_GUEST_PAGE_SIZE = Some(65536.0f64);
        ctx.globals.m_native__abi__TETO_PAGE_ENTRY_SIZE = Some(16.0f64);
        ctx.globals.m_native__abi__TETO_PAGE_CAPACITY = Some(8192.0f64);
        ctx.globals.m_native__abi__TETO_HART_EMPTY = Some(0.0f64);
        ctx.globals.m_native__abi__TETO_HART_RUNNABLE = Some(1.0f64);
        ctx.globals.m_native__abi__TETO_HART_WAITING = Some(2.0f64);
        ctx.globals.m_native__abi__TETO_HART_EXITED = Some(3.0f64);
        ctx.globals.m_native__abi__TETO_HART_FAULTED = Some(4.0f64);
        ctx.globals.m_native__abi__TETO_EVENT_NONE = Some(0.0f64);
        ctx.globals.m_native__abi__TETO_EVENT_SYSCALL = Some(1.0f64);
        ctx.globals.m_native__abi__TETO_EVENT_HOST = Some(2.0f64);
        ctx.globals.m_native__abi__TETO_HOST_NONE = Some(0.0f64);
        ctx.globals.m_native__abi__TETO_HOST_WRITE = Some(1.0f64);
        ctx.globals.m_native__abi__TETO_BATCH_BUDGET = Some(0.0f64);
        ctx.globals.m_native__abi__TETO_BATCH_SYSCALL = Some(1.0f64);
        ctx.globals.m_native__abi__TETO_BATCH_HOST = Some(2.0f64);
        ctx.globals.m_native__abi__TETO_BATCH_EXITED = Some(3.0f64);
        ctx.globals.m_native__abi__TETO_BATCH_FAULT = Some(4.0f64);
        ctx.globals.m_native__abi__TETO_BATCH_BUSY = Some(5.0f64);
        ctx.globals.m_native__abi__TETO_BATCH_CONTENDED = Some(6.0f64);
        ctx.globals.m_native__abi__TETO_FAULT_NONE = Some(0.0f64);
        ctx.globals.m_native__abi__TETO_FAULT_MEMORY = Some(1.0f64);
        ctx.globals.m_native__abi__TETO_FAULT_INSTRUCTION = Some(2.0f64);
        ctx.globals.m_native__abi__TETO_FAULT_BREAKPOINT = Some(3.0f64);
        ctx.globals.m_native__abi__TETO_FAULT_BAD_STATE = Some(4.0f64);
        ctx.globals.m_native__abi__TETO_THX_OK = Some(0.0f64);
        ctx.globals.m_native__abi__TETO_THX_BUSY = Some(1.0f64);
        ctx.globals.m_native__abi__TETO_THX_RANGE = Some(2.0f64);
        ctx.globals.m_native__abi__TETO_THX_MAGIC = Some(3.0f64);
        ctx.globals.m_native__abi__TETO_THX_LENGTH = Some(4.0f64);
        ctx.globals.m_native__abi__TETO_THX_CHECKSUM = Some(5.0f64);
        ctx.globals.m_native__abi__TETO_THX_JSON = Some(6.0f64);
        ctx.globals.m_native__abi__TETO_THX_METADATA = Some(7.0f64);
        ctx.globals.m_native__abi__TETO_THX_SECTION = Some(8.0f64);
        ctx.globals.m_native__abi__TETO_THX_MEMORY = Some(9.0f64);
        ctx.globals.m_native__abi__TETO_STARTUP_MAGIC = Some(827609940.0f64);
        ctx.globals.m_native__abi__TETO_START_OK = Some(0.0f64);
        ctx.globals.m_native__abi__TETO_START_RANGE = Some(1.0f64);
        ctx.globals.m_native__abi__TETO_START_FORMAT = Some(2.0f64);
        ctx.globals.m_native__abi__TETO_START_MEMORY = Some(3.0f64);
        ctx.globals.m_core__rv64_arithmetic__MASK32 = Some(4294967295i128);
        ctx.globals.m_native__word__MASK32 = Some(4294967295i128);
        ctx.globals.m_native__vfs__VFS_HEADER_SIZE = Some(32.0f64);
        ctx.globals.m_native__vfs__VFS_INODE_IMAGE_SIZE = Some(56.0f64);
        ctx.globals.m_native__vfs__VFS_DENTRY_IMAGE_SIZE = Some(16.0f64);
        ctx.globals.m_native__kernel__NO_RESERVATION = Some((1i128).checked_neg().expect("BigInt negation"));
        ctx.globals.m_native__kernel__INVALID_POINTER = Some(4294967295.0f64);
        ctx.globals.m_native__kernel__UNMAPPED_POINTER = Some(4294967294.0f64);
        ctx.globals.m_native__kernel__INVALID_PROCESS = Some(4294967295.0f64);
        ctx.globals.m_native__kernel__MAP_NOT_OWNED = Some((4096i128).checked_neg().expect("BigInt negation"));
        ctx.globals.m_native__kernel__MAP_PAGE = Some(4096i128);
        ctx.globals.m_native__kernel__MAP_GUARD = Some((1024i128).checked_mul(1024i128).expect("BigInt arithmetic"));
        ctx.globals.m_native__kernel__MAP_SPACING = Some(((16i128).checked_mul(1024i128).expect("BigInt arithmetic")).checked_mul(1024i128).expect("BigInt arithmetic"));
        ctx.globals.m_native__kernel__MMAP_TRACE_COUNTER = Some(1000.0f64);
        ctx.globals.m_native__kernel__MMAP_TRACE_BASE = Some(1024.0f64);
        ctx.globals.m_native__kernel__MMAP_TRACE_STRIDE = Some(40.0f64);
        ctx.globals.m_native__kernel__MMAP_TRACE_CAPACITY = Some(48.0f64);
        ctx.globals.m_native__kernel__MMAP_TRACE_START = Some(17179869184i128);
        ctx.globals.m_native__kernel__MMAP_TRACE_END = Some(21474836480i128);
        ctx.globals.m_native__kernel__OPEN_DIRECTORY = Some(65536.0f64);
        ctx.globals.m_native__kernel__OPEN_CLOEXEC = Some(524288.0f64);
        ctx.globals.m_native__start__START_BAD = Some(4294967295.0f64);
        ctx.globals.m_native__thx__BAD = Some(4294967295.0f64);
        ctx.globals.m_native__thx__MAX_SAFE = Some(9007199254740991i128);
        ctx.globals.m_native__thx__K_MACHINE = Some(1.0f64);
        ctx.globals.m_native__thx__K_VER = Some(2.0f64);
        ctx.globals.m_native__thx__K_SEC = Some(3.0f64);
        ctx.globals.m_native__thx__K_SYM = Some(4.0f64);
        ctx.globals.m_native__thx__K_REL = Some(5.0f64);
        ctx.globals.m_native__thx__K_DBG = Some(6.0f64);
        ctx.globals.m_native__thx__K_IDENT = Some(7.0f64);
        ctx.globals.m_native__thx__K_ENTRY = Some(8.0f64);
        ctx.globals.m_native__thx__K_MEM = Some(9.0f64);
        ctx.globals.m_native__thx__K_ISA = Some(10.0f64);
        ctx.globals.m_native__thx__K_PHDR = Some(11.0f64);
        ctx.globals.m_native__thx__K_PHENT = Some(12.0f64);
        ctx.globals.m_native__thx__K_PHNUM = Some(13.0f64);
        ctx.globals.m_native__thx__K_NAME = Some(14.0f64);
        ctx.globals.m_native__thx__K_FLG = Some(15.0f64);
        ctx.globals.m_native__thx__K_ALIGN = Some(16.0f64);
        ctx.globals.m_native__thx__K_SIZE = Some(17.0f64);
        ctx.globals.m_native__thx__K_ADDR = Some(18.0f64);
        ctx.globals.m_native__thx__K_AT = Some(19.0f64);
        ctx.globals.m_native__thx__K_LEN = Some(20.0f64);
        ctx
    }
}
