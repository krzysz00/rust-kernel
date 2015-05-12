        .globl gdt
        .globl gdtDesc
        .extern idtDesc
        .globl tmpIdtDesc
        .globl lidt2
gdt:
        .long 0, 0
        .long 0x0000FFFF, 0x00CF9a00
        .long 0x0000FFFF, 0x00CF9200
        .long 0x0000FFFF, 0x00CFFa00
        .long 0x0000FFFF, 0x00CFF200
        .rept 10 - 5
        .long 0, 0
        .endr
gdtEnd:
gdtDesc:
        .word (gdtEnd - gdt - 1)
        .long gdt
tmpIdtDesc:
        .word 0
        .long 0
lidt2:
        lidt idtDesc
        ret
