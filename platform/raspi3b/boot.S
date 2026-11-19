/* Raspberry Pi 3B AArch64 reset shim. This is Rei platform code, not Thistle. */
.section .text.boot, "ax"
.align 4
.global _start
.type _start, %function
_start:
    /* Run only on the primary Cortex-A53 core. */
    mrs x0, mpidr_el1
    and x0, x0, #3
    cbnz x0, .Lpark

    ldr x0, =__stack_top
    mov sp, x0

    /* Clear the uninitialised data area before calling C. */
    ldr x0, =__bss_start
    ldr x1, =__bss_end
.Lzero:
    cmp x0, x1
    b.hs .Lstart
    str xzr, [x0], #8
    b .Lzero

.Lstart:
    bl rei_raspi3b_start
.Lpark:
    wfe
    b .Lpark
.size _start, .-_start
