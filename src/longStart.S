        .globl longStart
        .extern k_main
longStart:
        cli
        # Enable SSE
        mov %cr0, %rax
        and $(~4), %rax
        or $2, %rax
        mov %rax, %cr0
        mov %cr4, %rax
        or $(1 << 9 | 1 << 10), %rax
        mov %rax, %cr4

        mov $stack_here, %rsp
        callq k_main
loop:
        jmp loop
end:
        .section .bss
stack_start:
        .skip 8192
stack_here:
        .skip 8
