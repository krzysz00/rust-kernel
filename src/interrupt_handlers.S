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
        mov 32(%esp), %eax
        push %eax
        cld
        call gpf_handler
        add $4, %esp
        popal
        add $4, %esp
        sti
        iret

page_fault_wrapper:
        pushal
        cld
        mov 32(%esp), %eax
        push %eax
        mov %cr2, %eax
        push %eax
        call page_fault_handler
        add $8, %esp
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

spurious_interrupt_handler:
        iret
