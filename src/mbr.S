/*
    This is the bootstrap code.

    It will be stored in sector#0 of the boot drive but the BIOS will load
    it at address 0x7c0

    We will compile the MBR at 0x7C00 then extract the appropriate 512 byte
    block and copy it to the boot drive

    
*/
   
        .code16

	.extern gdtDesc
	.extern idtDesc

	.global start
start:
	cli			# disable interrupts

	mov $0,%ax		# initialize data and stack segments
	mov %ax,%ds
	mov %ax,%ss

	movb %dl,0x7000		# save boot drive id

	# print an 'x' to let the world know we're here
	movb $'x',%al
	movb $1,%ah
	mov $0,%dx
	int $0x14

	# read the drive parameters
	movw $30,0x6000
	movb $0x48,%ah		# function code (read drive parameters)
	movb 0x7000,%dl		# drive index
	movw $0x6000,%si	# result buffer
	int $0x13		# call the BIOS

	movw 0x6010,%cx		# low 16 bits of sector count
				# this limits hdd size to 64k * 512 = 32MB


	# read hdd into memory starting at loadKernelHere
	movw $0x6000,%si	# DAP pointer
	movb $16,(%si)		# size of buffer
	movb $0,1(%si)		# unused
	add $-1,%cx		# -1 sector
	movw %cx,2(%si)		# number of sectors
	movw $loadKernelHere,4(%si)	# buffer offset
	movw $0,6(%si)		# buffer segment
	movw $1,8(%si)		# starting sector number
	movw $0,10(%si)
	movw $0,12(%si)
	movw $0,14(%si)

	mov $0x42,%ah		# function code
	movb 0x7000,%dl		# drive index
	int $0x13		# read the sectors

	# load the gdt
	lgdt gdtDesc

	# load the idt
	lidt idtDesc

	# enable protected mode, still running in 16 bit segment with
        # original segments
	mov %cr0,%eax
	or $1,%eax
	mov %eax,%cr0


	# set the stack and data segments, can't touch memory until
	# we get into 32 bit mode
	mov $16,%ax
	mov %ax,%ds
	mov %ax,%ss


	# do a far jump to the kernel entry point in 32 bit mode
	ljmp $8,$kStart


end:
	.skip 510 - (end - start)
	.byte 0x55
	.byte 0xaa

loadKernelHere:

