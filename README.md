# Building
You need a copy of the rust sources to cross-compile libcore and similar for the "new architecture"
We have to do that because `rustc` doesn't have a `-m32`.

The Makefile looks for these sources at `deps/rust` , but you can override that with `make RUSTSRC=directory`

Run this by loading `build/kernel.img` into `qemu`
