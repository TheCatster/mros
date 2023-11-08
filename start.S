.global _start
.extern long_start

.text
.code32
header_start:
	.long 0xe85250d6										/* multiboot2 */
	.long 0													/* x86 32-bit */
	.long header_end - header_start							/* header size */
	.long -(0xe85250d6 + 0 + (header_end - header_start))	/* checksum */

	.word 0
	.word 0
	.long 8													/* end tag */
header_end:

setup_page_table:
    movl _l4pgtbl, %eax
    movl _l3pgtbl, %ebx
    or $0b11, %ebx
    movl %ebx, (%eax)

    movl _l3pgtbl, %eax
    movl _l2pgtbl, %ebx
    or $0b11, %ebx
    movl %ebx, (%eax)

    movl $0, %ecx

.map_page_loop:
    movl $0x20000, %eax
    imul %ecx, %eax
    or $0x10000011, %eax
    movl _l2pgtbl, %ebx
    movl %eax, (%ebx, %ecx, 8)

    inc %ecx
    cmp $512, %ecx
    jne .map_page_loop

    ret

enable_paging:
    /* load level-4 pagetable to cr3 */
    movl _l4pgtbl, %eax
    movl %eax, %cr3

    /* set PAE-flag in cr4 */
    movl %cr4, %eax
    or $32, %eax
    movl %eax, %cr4

    /* set 64 mode in msr */
    movl 0xc0000080, %ecx
    rdmsr
    or $256, %eax
    wrmsr

    /* enable paging */
    movl %cr0, %eax
    or 1<<31, %eax
    movl %eax, %cr0

    ret

_start:
    movl kernel_stack, %esp
    /* set page table */
    call setup_page_table
    call enable_paging

    /* set global descriptor table */
    lgdt .pointer

    /* call rust main */
    jmp long_start

.section .bss
.align 4096
_l4pgtbl:
    .space 4096
_l3pgtbl:
    .space 4096
_l2pgtbl:
    .space 4096
kernel_stack:
    .space 4096

.section .rodata
gdt64:
    .quad 0
.code = . - gdt64
    .quad (1<<43) | (1<<44) | (1<<47) | (1<<53)
.pointer:
    .word . - gdt64 - 1
    .quad gdt64