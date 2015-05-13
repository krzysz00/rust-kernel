.PHONY: clean

vpath %.rs interrupts/
RUSTFLAGS += --target=../i686-unknown-elf.json -L. -L${DEPDIR} -g -C opt-level=3 -Z no-landing-pads

RUSTC = rustc
RUSTFILES = $(notdir $(wildcard *.rs) $(wildcard interrupts/*.rs))
SFILES = $(notdir $(wildcard *.S) $(wildcard *.s))
OFILES = $(subst .s,.o,$(subst .S,.o,$(SFILES)))
BOOTFILES = $(sort $(filter boot%,${OFILES}))
NON_BOOTFILES = $(filter-out boot%,${OFILES})

AFILES = libasmcode.a librustcode.a

AR = ar
LD = ld
OBJCOPY = objcopy
DD = dd

CC = gcc
ASFLAGS += -m32

%.o: %.S
	${CC} ${ASFLAGS} -c -o $@ $<

%.o: %.s
	${CC} ${ASFLAGS} -c -o $@ $<

libasmcode.a: ${OFILES}
	${AR} cr $@ ${NON_BOOTFILES}

librustcode.a: ${RUSTFILES}
	${RUSTC} ${RUSTFLAGS} lib.rs

%.bin: %
	${OBJCOPY} -O binary $< $@

%.img: %.bin
	${DD} if=$< of=$@ bs=512 conv=sync

clean::
	rm -f *.o
	rm -f *.a
	rm -f *.img
	rm -f *.bin
