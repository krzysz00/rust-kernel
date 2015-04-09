.PHONY: all
.PHONY: clean
.PHONY: dirs

ODIR = build
SRCDIR = src
DEPDIR = deps

vpath %.rs src
vpath %.rs deps
vpath %.S src
vpath % build

RUSTC = rustc
RUSTFLAGS_CORE = --target=i686-unknown-elf.json --cfg arch__x86
RUSTFLAGS += --out-dir=${ODIR}/ -L${ODIR} -g -C opt-level=3 --extern core=${DEPDIR}/libcore.rlib ${RUSTFLAGS_CORE}

RUSTFILES = $(notdir $(wildcard ${SRCDIR}/*.rs))
SFILES = $(notdir $(wildcard ${SRCDIR}/*.S) $(wildcard ${SRCDIR}/*.s))
OFILES = $(subst .s,.o,$(subst .S,.o,$(SFILES)))

AR = ar
LD = ld
OBJCOPY = objcopy
DD = dd

CC = gcc
ASFLAGS += -m32

all: kernel.img

clean:
	 rm -rfv ${ODIR}/

dirs: ${ODIR}

${ODIR}:
	mkdir -p $@

${DEPDIR}/libcore.rlib: i686-unknown-elf.json
	${RUSTC} ${RUSTFLAGS_CORE} --crate-type=lib -o $@ ${DEPDIR}/libcore/lib.rs

librlibc.rlib: rlibc.rs ${DEPDIR}/libcore.rlib build
	${RUSTC} ${RUSTFLAGS} --crate-type=rlib --crate-name=rlibc $<

%.o: %.S build
	${CC} ${ASFLAGS} -c -o ${ODIR}/$@ $<

%.o: %.s build
	${CC} ${ASFLAGS} -c -o $@ $<

libasmcode.a: ${OFILES}
	${AR} cr ${ODIR}/$@ $(addprefix ${ODIR}/,$(filter-out mbr.o,${OFILES}))

librustcode.a: librlibc.rlib libasmcode.a ${RUSTFILES} i686-unknown-elf.json
	${RUSTC} ${RUSTFLAGS} ${SRCDIR}/lib.rs

kernel: libasmcode.a librustcode.a
	${LD} -N -m elf_i386 -e start -Ttext=0x7c00 -o ${ODIR}/kernel ${ODIR}/mbr.o --start-group $(addprefix ${ODIR}/,$?) --end-group

%.bin: %
	${OBJCOPY} -O binary ${ODIR}/$< ${ODIR}/$@

%.img: %.bin
	${DD} if=${ODIR}/$< of=${ODIR}/$@ bs=512 conv=sync
