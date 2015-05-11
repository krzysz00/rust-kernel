# Building
You need a copy of the rust sources to cross-compile libcore and similar for the "new architecture"
We have to do that because `rustc` doesn't have a `-m32`.

The Makefile looks for these sources at `deps/rust` , but you can override that with `make RUSTSRC=directory`

The build is `-O3` by default, but you can edit the Makefile to change that

Run this by loading `build/kernel.img` into `qemu`

This code uses some unstable features (like inline assembly).
To use unstable features, you need a nightly build of the compiler.
Since tracking the nightly branch might be a bad idea, you can use `rustup.sh --channel=nightly --date=[date-of-relevant-stable-release]` to get as close to the stable build as possible.
When you do that, make sure to check out the corresponding source code.
(`rustup.sh`, which is Rust's installer script, also takes a `--prefix` option)
