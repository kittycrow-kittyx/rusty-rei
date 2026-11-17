# Raspberry Pi

This branch links the generated Rust kernel to Circle for Raspberry Pi 5.
The generated source in src/ remains identical to the platform-independent
kernel. The platform crate implements Crust's Platform trait using a bounded,
single-hart memory arena. It does not implement guest instructions or syscalls.

Circle and the C++ hardware adapter are pinned separately. The adapter is
taken from Rei's hardware branch, not from its C kernel. No Croissant source
or C kernel objects are compiled or linked. The shared adapter handles the
screen, serial, interrupts, timer, USB keyboard and onboard SD card.

## Build

Install the AArch64 bare-metal GNU toolchain and Rust. Use the same root.vfs
produced by the mikuOS Rei build.

    rustup target add aarch64-unknown-none
    REI_JOBS=2 nice -n 10 bash scripts/build-raspi5-circle.sh /path/to/root.vfs
    REI_JOBS=2 nice -n 10 bash scripts/build-raspi5-circle.sh /path/to/root.vfs sd

The outputs are build/native/raspi5-circle/rusty-rei-raspi5-circle.img
and build/native/raspi5-circle-sd/rusty-rei-raspi5-circle-sd.img.
The latter loads root.vfs from the onboard SD FAT volume without an
embedded fallback. The build checks the common Thistle, Bake and Circle
pins against the adapter dependency. The Rust library is compiled directly
from the checked-in source, without regenerating it.

## Acceptance

    git submodule update --init --jobs 2 -- thistle
    nice -n 10 cargo test --manifest-path platform/raspi5/Cargo.toml --lib -j 2

The host tests cover buffer identity, endian and signed integer access,
overlapping copies, wrapping atomic operations and bounds. They are not
a physical boot test. On the Pi, use the same SD card root archive and
firmware configuration as Rei, selecting the Rusty Rei image instead.
Expect RUSTY-REI: RASPI5 AARCH64 BOOT, the canonical kernel and root mount
markers, the real RV64GC hello output and successful guest exit, then
the shared SD and keyboard diagnostics. The SD-root variant must also
report SD ROOT READ PASS. Verify both variants locally before deployment.

No multicore guest execution is enabled. The smoke launcher does not have
guest stdin, real startup entropy or persistent writable storage. Circle
uses GPL-3.0 terms; review the obligations for linked image distribution.
Compilation and physical execution of these targets remain unverified.
