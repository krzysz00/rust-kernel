.PHONY: all
.PHONY: clean

ODIR = build
SRCDIR = src
DEPDIR = deps

vpath %.rs src
vpath %.rs deps
vpath %.S src
vpath %.s src
vpath %.c src
vpath % build

RUSTSRC = deps/rust/

RUSTC = rustc
RUSTFLAGS_CORE = --target=i686-unknown-elf.json --cfg arch__x86
RUSTFLAGS += --out-dir=${ODIR}/ -L${ODIR} -g -C opt-level=3 --extern core=${DEPDIR}/libcore.rlib ${RUSTFLAGS_CORE}

CFLAGS += -m32 -nostdlib -nostdinc -g -O3 -Wall -Werror

RUSTFILES = $(notdir $(wildcard ${SRCDIR}/*.rs))
SFILES = $(notdir $(wildcard ${SRCDIR}/*.S) $(wildcard ${SRCDIR}/*.s))
OFILES = mulodi4.o $(subst .s,.o,$(subst .S,.o,$(SFILES)))

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

${DEPDIR}/libcore.rlib: i686-unknown-elf.json
	${RUSTC} ${RUSTFLAGS_CORE} --crate-type=lib -o $@ ${RUSTSRC}/src/libcore/lib.rs

librlibc.rlib: rlibc.rs ${DEPDIR}/libcore.rlib | ${ODIR}/.timestamp
	${RUSTC} ${RUSTFLAGS} --crate-type=rlib --crate-name=rlibc $<

librlibm.rlib: rlibm.rs ${DEPDIR}/libcore.rlib | ${ODIR}/.timestamp
	${RUSTC} ${RUSTFLAGS} --crate-type=rlib --crate-name=rlibm $<

%.o: %.c | ${ODIR}/.timestamp
	gcc ${CFLAGS} -o build/$@ -c $<

%.o: %.S | ${ODIR}/.timestamp
	${CC} ${ASFLAGS} -c -o ${ODIR}/$@ $<

%.o: %.s | ${ODIR}/.timestamp
	${CC} ${ASFLAGS} -c -o $@ $<

libasmcode.a: ${OFILES}
	${AR} cr ${ODIR}/$@ $(addprefix ${ODIR}/,$(filter-out mbr.o,${OFILES}))

librustcode.a: ${RUSTFILES} librlibc.rlib librlibm.rlib libasmcode.a
	${RUSTC} ${RUSTFLAGS} ${SRCDIR}/lib.rs

kernel: ${AFILES}
	${LD} -N -m elf_i386 -e start -Ttext=0x7c00 -o ${ODIR}/kernel ${ODIR}/mbr.o --start-group $(addprefix ${ODIR}/,${AFILES}) --end-group -L /usr/lib/gcc/x86_64-linux-gnu/4.9/32 -lgcc

%.bin: %
	${OBJCOPY} -O binary ${ODIR}/$< ${ODIR}/$@

%.img: %.bin
	${DD} if=${ODIR}/$< of=${ODIR}/$@ bs=512 conv=sync
