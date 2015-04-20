        .globl smp_init_vector
        .extern gdtDesc
        .extern idtDesc
        .extern SMP_STACK_PTR
        .extern SMP_CR3
        .extern k_main
        .section .text
SMPStart:
        lidt idtDesc
        mov SMP_CR3, %eax
        mov %eax, %cr3
        mov %cr0, %eax
        or $0x80000000, %eax
        mov %eax, %cr0

        mov %cr4, %eax
        or $0x10, %eax
        mov %eax, %cr4

        mov SMP_STACK_PTR, %esp
        call k_main

        .code16
SMPBegin:
        mov $0, %ax
        mov %ax, %ds
        mov %ax, %ss

        lgdt gdtDesc

        mov %cr0, %eax
        or $1, %eax
        mov %eax, %cr0

        mov $0x10, %ax
        mov %ax, %ds
        mov %ax, %es
        mov %ax, %fs
        mov %ax, %gs
        mov %ax, %ss

        ljmp $8, $SMPStart
SMP_end:
        .skip 512 - (SMP_end - SMPStart)
smp_init_vector:
        ljmp $0, $SMPBegin
