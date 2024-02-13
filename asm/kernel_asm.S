/*
 * kernel_asm.S - kernel assembly routines (Assignment 1, CSE 597)
 * Copyright 2023 Ruslan Nikolaev <rnikola@psu.edu>
 */

.global default_trap, timer_apic
.code64

/*
 * These macros save and restore volatile registers
 * (assuming you do not modify any other registers except
 * as used by the C calling convention).
 */
#define SAVE_REGS					 \
	pushq %rax						;\
	pushq %rcx						;\
	pushq %rdx						;\
	pushq %rdi						;\
	pushq %rsi						;\
	pushq %r8						;\
	pushq %r9						;\
	pushq %r10						;\
	pushq %r11

#define RESTORE_REGS				 \
	popq %r11						;\
	popq %r10						;\
	popq %r9						;\
	popq %r8						;\
	popq %rsi						;\
	popq %rdi						;\
	popq %rdx						;\
	popq %rcx						;\
	popq %rax

.align 64
.type default_trap,%function
default_trap:
	cli
	SAVE_REGS
	/* TODO: Do something here */
	RESTORE_REGS
	hlt

.align 64
.type timer_apic,%function
timer_apic:
	cli
	SAVE_REGS
	/* TODO: Do something here */
	RESTORE_REGS
	sti
	iretq
