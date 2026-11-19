/* BCM2837 PL011 at 0x3f201000: Raspberry Pi 3B and QEMU raspi3b. */
#include <stdint.h>
#include "uart.h"

#define REI_PL011_BASE UINT64_C(0x3F201000)
#define REI_UART_DR 0x00u
#define REI_UART_FR 0x18u
#define REI_UART_IBRD 0x24u
#define REI_UART_FBRD 0x28u
#define REI_UART_LCRH 0x2cu
#define REI_UART_CR 0x30u
#define REI_UART_ICR 0x44u

static void write_reg(uint32_t offset, uint32_t value) {
    *(volatile uint32_t *)(uintptr_t)(REI_PL011_BASE + offset) = value;
}
static uint32_t read_reg(uint32_t offset) {
    return *(volatile uint32_t *)(uintptr_t)(REI_PL011_BASE + offset);
}
void rei_uart_init(void) {
    /* QEMU's BCM2837 UART clock is sufficient for this fixed divisor. */
    write_reg(REI_UART_CR, 0);
    write_reg(REI_UART_IBRD, 26);
    write_reg(REI_UART_FBRD, 3);
    write_reg(REI_UART_LCRH, 0x70);  /* 8 data bits, FIFO enabled */
    write_reg(REI_UART_ICR, 0x7ff);
    write_reg(REI_UART_CR, 0x301);  /* UART, transmitter and receiver */
}
static void putc_uart(char ch) {
    while ((read_reg(REI_UART_FR) & (1u << 5)) != 0) {
        __asm__ volatile("yield");
    }
    write_reg(REI_UART_DR, (uint32_t)(unsigned char)ch);
}
void rei_uart_puts(const char *message) {
    while (*message) {
        if (*message == '\n') putc_uart('\r');
        putc_uart(*message++);
    }
}
