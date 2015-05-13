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

        .globl syscall_handler
        .extern write_handler

        .macro deuser
        mov %ds, %eax
        push %eax
        mov $0x10, %eax
        mov %ax, %ds
        mov %ax, %es
        mov %ax, %fs
        mov %ax, %gs
        .endm

        .macro reuser
        pop %eax
        mov %ax, %ds
        mov %ax, %es
        mov %ax, %fs
        mov %ax, %gs
        .endm

double_fault_wrapper:
        pushal
        deuser
        cli
        cld
        call double_fault_handler
        reuser
        popal
_loop:      jmp _loop

gpf_wrapper:
        pushal
        deuser
        mov 36(%esp), %eax
        push %eax
        cld
        call gpf_handler
        add $4, %esp
        reuser
        popal
        add $4, %esp
        iret

page_fault_wrapper:
        pushal
        deuser
        cld
        mov 36(%esp), %eax
        push %eax
        mov %cr2, %eax
        push %eax
        call page_fault_handler
        add $8, %esp
        reuser
        popal
        add $4, %esp
        iret

kbd_interrupt_wrapper:
        pushal
        deuser
        cld
        call kbd_interrupt_handler
        reuser
        popal
        iret

spurious_interrupt_handler:
        iret

        # User, save your registers. They're toast
        # %eax = Syscall number
        # Args in %ecx, %edx, %ebx, (add more later)
syscall_handler:
        mov %eax, %esi
        deuser
        cmp $1, %esi
        jne not_write
        push %edx
        push %ecx
        call write_handler
        add $8, %esp
not_write:
        mov %eax, %esi
        reuser
        mov %esi, %eax
        iret
