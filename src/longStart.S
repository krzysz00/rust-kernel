#define STACK_ADDR 0x180000
        .globl longStart
        .extern k_main
longStart:
        cli
        mov %cr0, %rax
        and $(~4), %rax
        or $2, %rax
        mov %rax, %cr0
        mov %cr4, %rax
        or $(1 << 9 | 1 << 10), %rax
        mov %rax, %cr4
        mov $STACK_ADDR, %rsp
        callq k_main
loop:
        jmp loop
end:
