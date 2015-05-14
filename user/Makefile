.PHONY: all clean
DEPDIR = ../deps
-include ../common.mak

all: user.img

user: ${BOOTFILES} ${AFILES}
	${LD} --gc-sections -N -m elf_i386 -e start -Ttext=0x400000 -o user ${BOOTFILES} --start-group ${AFILES} --end-group

clean::
	rm -f user
