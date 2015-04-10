/*
 * This is where the magic assembly wrappers around interrupt handlers go
 */

        .globl double_fault_wrapper
        .extern double_fault_handler

        .globl gpf_wrapper
        .extern gpf_handler

        .globl kbd_interrupt_wrapper
        .extern kbd_interrupt_handler

double_fault_wrapper:
        pushal
        cli
        cld
        call double_fault_handler
        popal
_loop:      jmp _loop

gpf_wrapper:
        pushal
        cli
        cld
        mov -32(%esp), %eax
        push %eax
        call gpf_handler
        add $4, %esp
        popal
        add $4, %esp
        sti
        iret

kbd_interrupt_wrapper:
        pushal
        cli
        cld
        call kbd_interrupt_handler
        popal
        sti
        iret
