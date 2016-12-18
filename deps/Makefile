.PHONY: all clean

DEPDIR = .
RUSTC = rustc
RUSTFLAGS_LIBS = --target=../i686-unknown-elf.json -L. --crate-type=rlib -C opt-level=3 -C panic=abort

clean:
	rm -ff *.rlib

all: libcollections.rlib librlibc.rlib liballoc_shim.rlib

libcore.rlib: ../i686-unknown-elf.json
	${RUSTC} ${RUSTFLAGS_LIBS} ${RUSTSRC}/src/libcore/lib.rs

liballoc.rlib: libcore.rlib
	${RUSTC} ${RUSTFLAGS_LIBS} ${RUSTSRC}/src/liballoc/lib.rs

libstd_unicode.rlib: libcore.rlib
	${RUSTC} ${RUSTFLAGS_LIBS} ${RUSTSRC}/src/libstd_unicode/lib.rs

libcollections.rlib: liballoc.rlib libstd_unicode.rlib
	${RUSTC} ${RUSTFLAGS_LIBS} ${RUSTSRC}/src/libcollections/lib.rs

librlibc.rlib: rlibc.rs libcore.rlib
	${RUSTC} ${RUSTFLAGS_LIBS} $<

liballoc_shim.rlib: alloc_shim.rs libcore.rlib
	${RUSTC} ${RUSTFLAGS_LIBS} $<
