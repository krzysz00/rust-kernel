# Building
You need a copy of the rust sources to cross-compile libcore and similar for the "new architecture"
We have to do that because `rustc` doesn't have a `-m32`.

The Makefile looks for these sources at `deps/rust` , but you can override that with `make RUSTSRC=directory`

The build is `-O3` by default, but you can edit the Makefile to change that

Run this by loading `build/kernel.img` into `qemu`
