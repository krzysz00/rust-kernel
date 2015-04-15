        .globl gdt
        .globl gdtDesc
        .globl gdtDesc32

        .extern idtDesc
        .globl tmpIdtDesc
        .globl lidt2
gdt32:
        .long 0, 0
        .long 0x0000FFFF, 0x00CF9a00
        .long 0x0000FFFF, 0x00CF9200
gdt32End:
gdtDesc32:
        .word gdt32End - gdt32
        .long gdt32
tmpIdtDesc:
        .word 0
        .quad 0
lidt2:
        lidt idtDesc
        retq

        .section .data
gdt:
        .long 0, 0
        .long 0, 0x00209A00
        .long 0, 0x00009200
gdtEnd:
gdtDesc:
        .word gdtEnd - gdt
        .quad gdt
