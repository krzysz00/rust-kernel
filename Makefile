.PHONY: all deps clean clean-all all-kernel

# This is relative to deps/
RUSTSRC = rust/
export RUSTSRC

all: kernel user

deps:
	${MAKE} -C deps all

kernel: deps
	${MAKE} -C kernel all

user: deps
	${MAKE} -C user all

clean:
	${MAKE} -C kernel clean
	${MAKE} -C user clean

clean-all: clean
	${MAKE} -C deps clean
