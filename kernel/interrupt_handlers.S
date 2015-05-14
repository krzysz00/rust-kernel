/*
 * This is where the magic assembly wrappers around interrupt handlers go
 */

#define KERNEL_CR3 0x1000

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
    .extern sleep_handler
    .extern exit_handler

    .macro deuser
    mov %ds, %eax
    push %eax
    mov $0x10, %eax
    mov %ax, %ds
    mov %ax, %es
    mov %ax, %fs
    mov %ax, %gs
    mov %cr3, %eax
    push %eax
    .endm

    .macro kernel_paging
    mov $KERNEL_CR3, %eax
    mov %eax, %cr3
    .endm

    .macro reuser
    pop %eax
    mov %eax, %cr3
    pop %eax
    mov %ax, %ds
    mov %ax, %es
    mov %ax, %fs
    mov %ax, %gs
    .endm

    .macro save_context
    pushal
    deuser
    kernel_paging
    mov %esp, %eax
    push %eax
    .endm

    .macro restore_context
    add $4, %esp
    reuser
    popal
    .endm

double_fault_wrapper:
    save_context
    cli
    cld
    call double_fault_handler
    restore_context
_loop:      jmp _loop

gpf_wrapper:
    save_context
    mov 44(%esp), %eax
    push %eax
    cld
    call gpf_handler
    add $4, %esp
    restore_context
    add $4, %esp
    iret

page_fault_wrapper:
    save_context
    cld
    mov 44(%esp), %eax
    push %eax
    mov %cr2, %eax
    push %eax
    call page_fault_handler
    add $8, %esp
    restore_context
    add $4, %esp
    iret

kbd_interrupt_wrapper:
    save_context
    cld
    call kbd_interrupt_handler
    restore_context
    iret

spurious_interrupt_handler:
    iret

    # User, save your registers. They're toast
    # %eax = Syscall number
    # Args in %ecx, %edx, %ebx, (add more later)
syscall_handler:
    mov %eax, %esi
    pushal
    deuser
    mov %esp, %eax
    push %eax

    push %edx
    push %ecx
    cmp $1, %esi
    je write
    cmp $8, %esi
    je sleep
    cmp $9, %esi
    je exit
    jmp end
write:
    call write_handler
    jmp end
sleep:
    call sleep_handler
    jmp end
exit:
    call exit_handler
    jmp end
end:
    add $8, %esp
    restore_context # Return codes go in the context
    iret
