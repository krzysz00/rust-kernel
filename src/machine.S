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

