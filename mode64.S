.globl long_start

.section .text
.code64
long_start:
    movw $0, %ax
    movw %ax, %ss
    movw %ax, %ds
    movw %ax, %es
    movw %ax, %fs
    movw %ax, %gs
    .extern rust_main
    call rust_main