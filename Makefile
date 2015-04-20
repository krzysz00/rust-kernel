.PHONY: all
.PHONY: clean

ODIR = build
SRCDIR = src
DEPDIR = deps

vpath %.rs src
vpath %.rs src/interrupts/
vpath %.rs deps
vpath %.rlib deps
vpath %.S src
vpath %.s src
vpath % build

RUSTSRC = deps/rust/

RUSTC = rustc
RUSTFLAGS_LIBS = --target=x86_64-unknown-elf.json --out-dir=${DEPDIR} -L${DEPDIR} --crate-type=rlib -C opt-level=3 -Z no-landing-pads
RUSTFLAGS += --target=x86_64-unknown-elf.json --out-dir=${ODIR} -L${ODIR} -L${DEPDIR} -g -C opt-level=3 -Z no-landing-pads

RUSTFILES = $(notdir $(wildcard ${SRCDIR}/*.rs) $(wildcard ${SRCDIR}/interrupts/*.rs))
SFILES = $(notdir $(wildcard ${SRCDIR}/*.S) $(wildcard ${SRCDIR}/*.s))
OFILES = $(subst .s,.o,$(subst .S,.o,$(SFILES)))
BOOTFILES = $(sort $(filter boot%,${OFILES}))
NON_BOOTFILES = $(filter-out boot%,${OFILES})

AFILES = libasmcode.a librustcode.a

AR = ar
LD = ld
OBJCOPY = objcopy
DD = dd

CC = gcc
ASFLAGS += -m64

all: kernel.img

clean:
	rm -rfv ${ODIR}/

${ODIR}/.timestamp:
	mkdir -p ${ODIR} && touch $@

libcore.rlib: x86_64-unknown-elf.json
	${RUSTC} ${RUSTFLAGS_LIBS} ${RUSTSRC}/src/libcore/lib.rs

liballoc.rlib: libcore.rlib
	${RUSTC} ${RUSTFLAGS_LIBS} --cfg feature=\"external_funcs\" ${RUSTSRC}/src/liballoc/lib.rs

librustc_unicode.rlib: libcore.rlib
	${RUSTC} ${RUSTFLAGS_LIBS} ${RUSTSRC}/src/librustc_unicode/lib.rs

libcollections.rlib: liballoc.rlib librustc_unicode.rlib
	${RUSTC} ${RUSTFLAGS_LIBS} ${RUSTSRC}/src/libcollections/lib.rs

librlibc.rlib: rlibc.rs libcore.rlib
	${RUSTC} ${RUSTFLAGS_LIBS} $<

%.o: %.S | ${ODIR}/.timestamp
	${CC} ${ASFLAGS} -c -o ${ODIR}/$@ $<

%.o: %.s | ${ODIR}/.timestamp
	${CC} ${ASFLAGS} -c -o $@ $<

libasmcode.a: ${OFILES}
	${AR} cr ${ODIR}/$@ $(addprefix ${ODIR}/, ${NON_BOOTFILES})

librustcode.a: ${RUSTFILES} librlibc.rlib libcollections.rlib
	${RUSTC} ${RUSTFLAGS} ${SRCDIR}/lib.rs

kernel: ${BOOTFILES} ${AFILES}
	${LD} --gc-sections -N -m elf_x86_64 -z max-page-size=0x1000 -e start -Ttext=0x7c00 -o ${ODIR}/kernel $(addprefix ${ODIR}/, ${BOOTFILES}) --start-group $(addprefix ${ODIR}/,${AFILES}) --end-group

%.bin: %
	${OBJCOPY} -O binary ${ODIR}/$< ${ODIR}/$@

%.img: %.bin
	${DD} if=${ODIR}/$< of=${ODIR}/$@ bs=512 conv=sync
