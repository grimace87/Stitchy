
# Stitchy CLI

![example workflow](https://github.com/grimace87/Stitchy/actions/workflows/cargo.yml/badge.svg)
[![Crates.io](https://img.shields.io/crates/v/stitchy.svg)](https://crates.io/crates/stitchy)

Joins multiple existing image files into a single output. Supports various flags for
arranging the output, choosing the image format, or scaling down to desirable dimensions.

This crate is a CLI tool which wraps the features of the
[stitchy-core](https://github.com/grimace87/Stitchy/tree/master/crates/stitchy-core) crate, adding configuration
capabilities using command-line arguments and profile defaults, and adding file output.

### Running

Performed with a simple command that operates on the current directory:

`stitchy n`

where `n` is the number of images you would like to stitch together into one. The tool
will take the `n` most recent files and arrange them in a grid, resizing where needed,
and outputting a file "stitch.jpg".

Various flags exist to adjust how source images are selected and how the output is generated.
Run `stitchy --help` to see a list of these flags.

### Installing

- Install Rust and Cargo if you don't already have them - see the official documentation
  at https://www.rust-lang.org/tools/install to install `rustup` and `cargo` and set up a
  toolchain
- Run `cargo install stitchy`

That's it! The binary will be built from source and placed in the `.cargo/bin` directory
inside your home folder. This should be available on your PATH if Cargo is installed
correctly.

Note that this crate will only compile with Rust version 1.56 and up.

#### NetBSD

If you are using NetBSD, `stitchy` is available from the official repositories.
Simply run,

```sh
pkgin install stitchy
```

to install the binary package

### Details

- Input files may be in JPEG, PNG, GIF or BMP format, and are taken from the current directory.
  The output format will match the source images if they are all the same, or default to JPEG
  otherwise. Flags can be used to require a particular output format.
- Compatible with Windows, macOS, Linux and NetBSD.
