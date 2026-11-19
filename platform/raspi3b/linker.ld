/* Raspberry Pi 3B firmware/QEMU AArch64 entry address. */
ENTRY(_start)
SECTIONS
{
  . = 0x80000;

  .text : ALIGN(16) {
    KEEP(*(.text.boot))
    *(.text .text.*)
  }

  .rodata : ALIGN(16) {
    *(.rodata .rodata.*)
  }

  .data : ALIGN(16) {
    *(.data .data.*)
  }

  .bss (NOLOAD) : ALIGN(16) {
    __bss_start = .;
    *(.bss .bss.*)
    *(COMMON)
    . = ALIGN(16);
    __bss_end = .;
  }

  . = ALIGN(16);
  . += 0x8000;
  __stack_top = .;

  /DISCARD/ : { *(.comment*) *(.note*) *(.eh_frame*) }
}
