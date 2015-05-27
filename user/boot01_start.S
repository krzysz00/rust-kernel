    # The PID is on the stack
    .globl start
start:
    call main
    mov $9, %eax
    mov $0, %ecx
    mov $0, %edx
    int $0x50
