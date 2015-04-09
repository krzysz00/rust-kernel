.PHONY: all
.PHONY: clean

ODIR = build
SRCDIR = src
DEPDIR = deps

vpath %.rs src
vpath %.rs deps
vpath %.S src
vpath % build

RUSTSRC = deps/rust/

RUSTC = rustc
RUSTFLAGS_CORE = --target=i686-unknown-elf.json --cfg arch__x86
RUSTFLAGS += --out-dir=${ODIR}/ -L${ODIR} -g -C opt-level=3 --extern core=${DEPDIR}/libcore.rlib ${RUSTFLAGS_CORE}

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
	echo "test" ${RUSTFILES}
	rm -rfv ${ODIR}/

${ODIR}/.timestamp:
	mkdir -p ${ODIR} && touch $@

${DEPDIR}/libcore.rlib: i686-unknown-elf.json
	${RUSTC} ${RUSTFLAGS_CORE} --crate-type=lib -o $@ ${RUSTSRC}/src/libcore/lib.rs

librlibc.rlib: rlibc.rs ${DEPDIR}/libcore.rlib | ${ODIR}/.timestamp
	${RUSTC} ${RUSTFLAGS} --crate-type=rlib --crate-name=rlibc $<

%.o: %.S | ${ODIR}/.timestamp
	${CC} ${ASFLAGS} -c -o ${ODIR}/$@ $<

%.o: %.s | ${ODIR}/.timestamp
	${CC} ${ASFLAGS} -c -o $@ $<

libasmcode.a: ${OFILES}
	${AR} cr ${ODIR}/$@ $(addprefix ${ODIR}/,$(filter-out mbr.o,${OFILES}))

librustcode.a: ${RUSTFILES} librlibc.rlib libasmcode.a
	${RUSTC} ${RUSTFLAGS} ${SRCDIR}/lib.rs

kernel: ${AFILES}
	${LD} -N -m elf_i386 -e start -Ttext=0x7c00 -o ${ODIR}/kernel ${ODIR}/mbr.o --start-group $(addprefix ${ODIR}/,${AFILES}) --end-group

%.bin: %
	${OBJCOPY} -O binary ${ODIR}/$< ${ODIR}/$@

%.img: %.bin
	${DD} if=${ODIR}/$< of=${ODIR}/$@ bs=512 conv=sync
