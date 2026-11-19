/* First native Rei Pi 3B milestone: run real Croissant-generated Thistle code.
 * This is NOT the full Thistle kernel or a bootable userspace.
 */
#include <stdint.h>
#include "croissant/core/rv64-branch.h"
#include "uart.h"

void rei_raspi3b_start(void) {
    rei_uart_init();
    rei_uart_puts("REI: RASPI3B AARCH64 BOOT\n");

    /* The implementation lives in src/core/rv64-branch.c generated
     * by Croissant from the pinned canonical Thistle source. */
    int passed =
        croissant_core_rv64_branch_ts__rv64Branch(0, 42, 42) == 1 &&
        croissant_core_rv64_branch_ts__rv64Branch(1, 42, 41) == 1 &&
        croissant_core_rv64_branch_ts__rv64Branch(4, -7, 8) == 1 &&
        croissant_core_rv64_branch_ts__rv64Branch(5, 8, 8) == 1 &&
        croissant_core_rv64_branch_ts__rv64Branch(3, 8, 8) == -1;

    if (passed) {
        rei_uart_puts("REI: THISTLE GENERATED CORE PASS\n");
    } else {
        rei_uart_puts("REI: THISTLE GENERATED CORE FAIL\n");
    }

    /* No scheduler, interrupts or platform idle implementation yet. */
    for (;;) __asm__ volatile("wfe");
}
