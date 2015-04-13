        .globl gdt
        .globl gdtDesc
        .extern idtDesc
        .globl tmpIdtDesc
        .globl lidt2
gdt:
        .long 0, 0
        .long 0x0000FFFF, 0x00CF9a00
        .long 0x0000FFFF, 0x00CF9200
        .rept 10 - 3
        .long 0, 0
        .endr
gdtEnd:
gdtDesc:
        .word (gdtEnd - gdt)
        .long gdt
tmpIdtDesc:
        .word 0
        .long 0
lidt2:
        lidt idtDesc
