	#
	# void ltr(uint32_t tr)
	#
	.global _ltr
_ltr:
	mov 4(%esp),%eax
	ltr %ax
	ret

        .globl _enable_paging
_enable_paging:
        mov 4(%esp), %eax
        mov %eax, %cr3
        mov %cr0, %eax
        or $0x80000000, %eax
        mov %eax, %cr0

        mov %cr4, %eax
        or $0x10, %eax
        mov %eax, %cr4
        ret

        .globl _rdmsr
        # uint64_t _rdmsr(uint32_t id)
_rdmsr:
        mov 4(%esp), %ecx
        rdmsr
        ret

        .globl _wrmsr
        # _wrmsr (uint32_t id, uint64_t value)
_wrmsr:
        mov 4(%esp), %ecx
        mov 8(%esp), %eax
        mov 12(%esp), %edx
        wrmsr
        ret

        .globl _invlpg
_invlpg:
        mov 4(%esp), %eax
        invlpg (%eax)
        ret

        # _to_user_mode(uint32_t entry)
        # Never returns
        .globl _to_user_mode
_to_user_mode:
        mov $0x23, %eax # USer data segment
        mov %ax, %ds
        mov %ax, %es
        mov %ax, %fs
        mov %ax, %gs

        mov %esp, %eax
        mov 4(%esp), %ecx
        push $0x23 # Stack segment
        push %eax # Stack pointer
        pushf
        push $0x1B # User code segment
        push %ecx # Handler
        iret

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
