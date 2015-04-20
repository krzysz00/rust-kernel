        .globl smp_init_vector
        .extern gdtDesc
        .extern idtDesc
        .extern SMP_STACK_PTR
        .extern SMP_CR3
        .extern k_main
        .section .text
SMPLongStart:
        cli
        # Enable SSE
        mov %cr0, %rax
        and $(~4), %rax
        or $2, %rax
        mov %rax, %cr0
        mov %cr4, %rax
        or $(1 << 9 | 1 << 10), %rax
        mov %rax, %cr4

        lidt idtDesc

        mov SMP_STACK_PTR, %rsp
        callq k_main

        .code32
SMPStart:
        mov SMP_CR3, %eax
        mov %eax, %cr3

        # Enable PAE, 4 MB pages, and the Global bit
        mov %cr4, %eax
        or $(1 << 4 | 1 << 5 | 1 << 7), %eax
        mov %eax, %cr4

        # Enable long mode
        mov $0xC0000080, %ecx
        rdmsr
        or $(1 << 8 | 1 << 11), %eax
        wrmsr

        mov %cr0, %eax
        or $(1 << 31), %eax
        mov %eax, %cr0

        lgdt gdtDesc
        ljmp $8, $SMPLongStart

        .code16
SMPBegin:
        cli
        mov $0, %ax
        mov %ax, %ds
        mov %ax, %ss

        lgdt gdtDesc32

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
        .skip 512 - (SMP_end - SMPLongStart)
smp_init_vector:
        ljmp $0, $SMPBegin
