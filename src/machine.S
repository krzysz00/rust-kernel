	# outb(int port, int val)
	.global _outb
_outb:
	push %edx
	mov 8(%esp),%dx
	mov 12(%esp),%al
	outb %al,%dx
	pop %edx
	ret

	# int inb(int port)
	.global _inb
_inb:
	push %edx
	mov 8(%esp),%dx
	inb %dx,%al
	pop %edx
	and $0xff,%eax
	ret

	# unsigned long inb(int port)
	.global _inl
_inl:
	push %edx
	mov 8(%esp),%dx
	inl %dx,%eax
	pop %edx
	ret


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

_invlpg:
        mov 4(%esp), %eax
        invlpg (%eax)
        ret

