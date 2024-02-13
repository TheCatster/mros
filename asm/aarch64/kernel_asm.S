/*
 * kernel_asm.S - kernel assembly routines (based on Assignment 1, CSE 597)
 * Copyright 2024 Daniil Rose <daniil.rose@posteo.org>
 */

.macro SAVE_REGS
    stp x0, x1, [sp, #-16]!
    stp x2, x3, [sp, #-16]!
    stp x4, x5, [sp, #-16]!
    stp x6, x7, [sp, #-16]!
    stp x8, x9, [sp, #-16]!
    stp x10, x11, [sp, #-16]!
    stp x12, x13, [sp, #-16]!
    stp x14, x15, [sp, #-16]!
    stp x16, x17, [sp, #-16]!
    stp x18, x29, [sp, #-16]!
    stp x30, lr, [sp, #-16]!
.endm

.macro RESTORE_REGS
    ldp x30, lr, [sp], #16
    ldp x18, x29, [sp], #16
    ldp x16, x17, [sp], #16
    ldp x14, x15, [sp], #16
    ldp x12, x13, [sp], #16
    ldp x10, x11, [sp], #16
    ldp x8, x9, [sp], #16
    ldp x6, x7, [sp], #16
    ldp x4, x5, [sp], #16
    ldp x2, x3, [sp], #16
    ldp x0, x1, [sp], #16
.endm

default_trap:
    // ??? This is my best attempt so far
    // Disable interrupts (modify DAIF)
    msr daifset, 0xf
    SAVE_REGS
    /* TODO: Do something here */
    RESTORE_REGS
    // Re-enable interrupts (modify DAIF) - if applicable
    msr daifclr, 0xf
    // Return from exception
    eret
