/*
 * This is the 32 bit kernel entry point
 *
 * In protected mode
 * At address 0x9000
 * Paging is disabled
 * ESP is not valid
 * Temporary GDT
 * BIOS IDT, TR
 *
 */

	.global kStart
    .extern kMain
kStart:
        mov $0x10, %eax
set_up_segments:
        mov %ax, %ds
        mov %ax, %es
        mov %ax, %fs
        mov %ax, %gs
        mov %ax, %ss
set_up_stack:
        movl $stack_here, %esp
        call kMain
        hlt // This puts the processor to sleep
end:
        .skip 4096
stack_here:
        .long 0xDEADBEEF
