.PHONY: all
.PHONY: clean

ODIR = build
SRCDIR = src
DEPDIR = deps

vpath %.rs src
vpath %.rs deps
vpath %.rlib deps
vpath %.S src
vpath %.s src
vpath % build

RUSTSRC = deps/rust/

RUSTC = rustc
RUSTFLAGS_LIBS = --target=i686-unknown-elf.json --out-dir=${DEPDIR} -L${DEPDIR} --crate-type=rlib -Z no-landing-pads
RUSTFLAGS += --target=i686-unknown-elf.json --out-dir=${ODIR} -L${ODIR} -L${DEPDIR} -g -C opt-level=3 -Z no-landing-pads

RUSTFILES = $(notdir $(wildcard ${SRCDIR}/*.rs))
SFILES = $(notdir $(wildcard ${SRCDIR}/*.S) $(wildcard ${SRCDIR}/*.s))
OFILES = $(subst .s,.o,$(subst .S,.o,$(SFILES)))

AFILES = libasmcode.a librustcode.a

AR = ar
LD = ld
OBJCOPY = objcopy
DD = dd

CC = gcc
ASFLAGS += -m32

all: kernel.img

clean:
	rm -rfv ${ODIR}/

${ODIR}/.timestamp:
	mkdir -p ${ODIR} && touch $@

libcore.rlib: i686-unknown-elf.json
	${RUSTC} ${RUSTFLAGS_LIBS} ${RUSTSRC}/src/libcore/lib.rs

liballoc.rlib: libcore.rlib
	${RUSTC} ${RUSTFLAGS_LIBS} --cfg feature=\"external_funcs\" ${RUSTSRC}/src/liballoc/lib.rs

libunicode.rlib: libcore.rlib
	${RUSTC} ${RUSTFLAGS_LIBS} ${RUSTSRC}/src/libunicode/lib.rs

libcollections.rlib: liballoc.rlib libunicode.rlib
	${RUSTC} ${RUSTFLAGS_LIBS} ${RUSTSRC}/src/libcollections/lib.rs

librlibc.rlib: rlibc.rs libcore.rlib
	${RUSTC} ${RUSTFLAGS_LIBS} $<

%.o: %.S | ${ODIR}/.timestamp
	${CC} ${ASFLAGS} -c -o ${ODIR}/$@ $<

%.o: %.s | ${ODIR}/.timestamp
	${CC} ${ASFLAGS} -c -o $@ $<

libasmcode.a: ${OFILES}
	${AR} cr ${ODIR}/$@ $(addprefix ${ODIR}/,$(filter-out mbr.o,${OFILES}))

librustcode.a: ${RUSTFILES} librlibc.rlib liballoc.rlib
	${RUSTC} ${RUSTFLAGS} ${SRCDIR}/lib.rs

kernel: ${AFILES}
	${LD} --gc-sections -N -m elf_i386 -e start -Ttext=0x7c00 -o ${ODIR}/kernel ${ODIR}/mbr.o --start-group $(addprefix ${ODIR}/,${AFILES}) --end-group

%.bin: %
	${OBJCOPY} -O binary ${ODIR}/$< ${ODIR}/$@

%.img: %.bin
	${DD} if=${ODIR}/$< of=${ODIR}/$@ bs=512 conv=sync
