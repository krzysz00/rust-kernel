.PHONY: all clean

DEPDIR = .
RUSTC = rustc
RUSTFLAGS_LIBS = --target=../i686-unknown-elf.json -L. --crate-type=rlib -C opt-level=3 -Z no-landing-pads

clean:
	rm -ff *.rlib

all: libcollections.rlib librlibc.rlib

libcore.rlib: ../i686-unknown-elf.json
	${RUSTC} ${RUSTFLAGS_LIBS} ${RUSTSRC}/src/libcore/lib.rs

liballoc.rlib: libcore.rlib
	${RUSTC} ${RUSTFLAGS_LIBS} --cfg feature=\"external_funcs\" ${RUSTSRC}/src/liballoc/lib.rs

librustc_unicode.rlib: libcore.rlib
	${RUSTC} ${RUSTFLAGS_LIBS} ${RUSTSRC}/src/librustc_unicode/lib.rs

libcollections.rlib: liballoc.rlib librustc_unicode.rlib
	${RUSTC} ${RUSTFLAGS_LIBS} ${RUSTSRC}/src/libcollections/lib.rs

librlibc.rlib: rlibc.rs libcore.rlib
	${RUSTC} ${RUSTFLAGS_LIBS} $<
