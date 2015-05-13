.PHONY: all clean
DEPDIR = ../deps
-include ../common.mak

all: kernel.img

kernel: ${BOOTFILES} ${AFILES}
	${LD} --gc-sections -N -m elf_i386 -e start -Ttext=0x7c00 -o kernel ${BOOTFILES} --start-group ${AFILES} --end-group

clean::
	rm -f kernel
