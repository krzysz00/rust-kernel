.PHONY: all deps clean clean-all all-kernel

RUSTSRC = deps/rust/
export RUSTSRC

all: all-kernel

deps:
	${MAKE} -C deps all

all-kernel: deps
	${MAKE} -C kernel all

clean:
	${MAKE} -C kernel clean

clean-all: clean
	${MAKE} -C deps clean
