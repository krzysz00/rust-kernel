    .globl gdt
    .globl gdt_desc
    .extern idt_desc
    .globl tmp_idt_desc
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
gdt_end:
gdt_desc:
    .word (gdt_end - gdt - 1)
    .long gdt
tmp_idt_desc:
    .word 0
    .long 0
lidt2:
    lidt idt_desc
    ret
