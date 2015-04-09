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


