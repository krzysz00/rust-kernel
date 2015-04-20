	# outb(int port, int val)
	.global _outb
_outb:
	mov %rdi, %rdx
	mov %rsi, %rax
	outb %al, %dx
	retq

	# int inb(int port)
	.global _inb
_inb:
	mov %rdi, %rdx
	inb %dx, %al
	and $0xff, %rax
	retq

	# unsigned long inb(int port)
	.global _inl
_inl:
	mov %rdi, %rdx
	inl %dx, %eax
	retq


	#
	# void ltr(uint32_t tr)
	#
	.global _ltr
_ltr:
	mov %rdi, %rax
	ltr %ax
	retq

        .globl _rdmsr
        # uint64_t _rdmsr(uint32_t id)
_rdmsr:
        mov %edi, %ecx
        rdmsr
        shl $32, %rdx
        or %rdx, %rax
        retq

        .globl _wrmsr
        # _wrmsr (uint32_t id, uint64_t value)
_wrmsr:
        mov %rdi, %rcx
        mov %rsi, %rax
        shr $32, %rsi
        mov %rsi, %rdx
        wrmsr
        retq

_invlpg:
        invlpg (%rdi)
        retq

