#![cfg_attr(not(test), no_std)]

mod memory;

#[cfg(test)]
fn hardware_sqrt(value: f64) -> f64 { value.sqrt() }

#[cfg(not(test))]
fn hardware_sqrt(value: f64) -> f64 {
    // AArch64 floating point is enabled by Circle startup.
    let result: f64;
    unsafe {
        core::arch::asm!("fsqrt {result:d}, {value:d}",
            result = out(vreg) result, value = in(vreg) value,
            options(pure, nomem, nostack));
    }
    result
}

#[cfg(not(test))]
mod boot;
