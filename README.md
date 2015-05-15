# Building
You need a copy of the rust sources to cross-compile libcore and
similar for the "new architecture" We have to do that because `rustc`
doesn't have a `-m32`. These sources need to be the same (or close
to), the sources used to build your `rustc` (see the commit hash in
`rustc --version`).

The Makefile looks for these sources at `deps/rust` , but you can
override that with `make RUSTSRC=directory`

The build is `-O3` by default, but you can edit the Makefile to change
that. (Note, when debugging, you probably want `-O1`, since, with
`rustc` `-O0` is "pessimized")

Run this by loading `kernel/kernel.img` and `user/user.img` into `qemu`
The kernel should be the first hard drive (hda).
The user program should be provided as the second hard drive (hdb).

This code uses some unstable features (like inline assembly). To use
unstable features, you need a nightly build of the compiler. Since
tracking the nightly branch might be a bad idea, you can use
`rustup.sh --channel=nightly --date=[date-of-relevant-beta-release]`
to get as close to the beta build as possible. When you do that,
make sure to check out the corresponding source code. (`rustup.sh`,
which is Rust's installer script, also takes a `--prefix` option).

(*NOTE*: 1.0-stable is not the sources from the day of release.
There's been an internal library renamed in the meantime, and some
other changes relevant to this project. Please track nightly until
1.1-beta)
