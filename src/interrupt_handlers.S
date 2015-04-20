/*
 * This is where the magic assembly wrappers around interrupt handlers go
 */

        .globl double_fault_wrapper
        .extern double_fault_handler

        .globl gpf_wrapper
        .extern gpf_handler

        .globl page_fault_wrapper
        .extern page_fault_handler

        .globl kbd_interrupt_wrapper
        .extern kbd_interrupt_handler

        .globl spurious_interrupt_handler

        .macro pushregs
        pushq %rax
        pushq %rbx
        pushq %rcx
        pushq %rdx
        pushq %rsp
        pushq %rbp
        pushq %rsi
        pushq %rdi
        pushq %r8
        pushq %r9
        pushq %r10
        pushq %r11
        pushq %r12
        pushq %r13
        pushq %r14
        pushq %r15
        .endm

        .macro popregs
        popq %r15
        popq %r14
        popq %r13
        popq %r12
        popq %r11
        popq %r10
        popq %r9
        popq %r8
        popq %rdi
        popq %rsi
        popq %rbp
        popq %rsp
        popq %rdx
        popq %rcx
        popq %rbx
        popq %rax
        .endm

double_fault_wrapper:
        pushregs
        cli
        cld
        call double_fault_handler
        popregs
_loop:      jmp _loop

gpf_wrapper:
        pushregs
        mov 128(%rsp), %rdi
        cld
        call gpf_handler
        popregs
        add $8, %esp
        iretq

page_fault_wrapper:
        pushregs
        cld
        mov 128(%rsp), %rsi
        mov %cr2, %rdi
        call page_fault_handler
        popregs
        add $8, %rsp
        iretq

kbd_interrupt_wrapper:
        pushregs
        cld
        call kbd_interrupt_handler
        popregs
        iretq

spurious_interrupt_handler:
        iretq
