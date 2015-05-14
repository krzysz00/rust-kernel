    # uint32_t _syscall(uint32_t number, uint32_t arg1, uint32_t arg2)
    .globl _syscall
_syscall:
    mov 4(%esp), %eax
    mov 8(%esp), %ecx
    mov 12(%esp), %edx
    push %ebx
    push %esi
    push %edi
    push %ebp
    int $0x50
    pop %ebp
    pop %edi
    pop %esi
    pop %ebx
    ret
