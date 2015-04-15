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
#define PAGING_START 0x1000
#define FRAME_SIZE 0x1000

        .section .text
        .code32
        .global kStart
        .extern longStart
kStart:
        cld
        # [0x1000, 0x5000] will be liberated
        mov $PAGING_START, %edi
        mov %edi, %cr3
        xor %eax, %eax
        mov $4096, %ecx
        rep stosl
        mov %cr3, %edi

        movl $0x2003, (%edi)
        add $FRAME_SIZE, %edi
        movl $(PAGING_START | 0x03), -8(%edi)
        movl $0x3003, (%edi)
        add $FRAME_SIZE, %edi
        movl $0x4003, (%edi)
        add $FRAME_SIZE, %edi # %edi is the first page table, entry 0

        mov $0x3, %ebx
        mov $512, %ecx
setEntry:
        mov %ebx, (%edi)
        add $FRAME_SIZE, %ebx
        add $8, %edi
        loop setEntry # loop does %ecx-- and jumpt to setEntry unless %ecx == 0

        # Enable PAE, 4 MB pages, and the Global bit
        mov %cr4, %eax
        or $(1 << 4 | 1 << 5 | 1 << 7), %eax
        mov %eax, %cr4

        # Enable long mode
        mov $0xC0000080, %ecx
        rdmsr
        or $(1 << 8 | 1 << 11), %eax
        wrmsr

        # Enable paging
        mov %cr0, %eax
        or $(1 << 31), %eax
        mov %eax, %cr0

        # Load the 64-bit stuff
        lgdt gdtDesc
        lidt tmpIdtDesc
        ljmp $0x08, $longStart
