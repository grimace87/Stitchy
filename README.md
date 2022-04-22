
# Stitchy

![example workflow](https://github.com/grimace87/Stitchy/actions/workflows/cargo.yml/badge.svg)
[![Crates.io](https://img.shields.io/crates/v/stitchy.svg)](https://crates.io/crates/stitchy)

Joins multiple existing image files into a single output. Supports various flags for
arranging the output or scaling down to desirable dimensions.

Images of the same size will stitch together neatly:

| Input files | | | Output |
| --- | --- | --- | --- |
| ![Sample 1](./images/demo/Screen1.jpg) | ![Sample 2](./images/demo/Screen2.jpg) | ![Sample 3](./images/demo/Screen3.jpg) | ![Sample Output](./images/demo/ScreenStitch.jpg) |

If the images are more irregular in shape, the tool will attempt to arrange them
as neatly as possible, and scale some images in the process:

| Input files | | | | Output |
| --- | --- | --- | --- | --- |
| ![Sample 1](./images/demo/Tree1.jpg) | ![Sample 2](./images/demo/Tree2.jpg) | ![Sample 3](./images/demo/Tree3.jpg) | ![Sample 3](./images/demo/Tree4.jpg) | ![Sample Output](./images/demo/TreeStitch.jpg) |

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
- Compatible with Windows (MSVC toolchain), macOS, Linux and NetBSD.
