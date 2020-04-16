
# Stitchy

Joins multiple existing image files into a single output.

### Running

Performed with a simple command that operates on the current directory:

`stitchy n`

where `n` is the number of images you would like to stitch together into one. The tool
will take the `n` most recent files and arrange them in a grid, resizing where needed,
and outputting a file "stitch.jpg".

### Building

This project is a Rust binary project, intended to be used as a command-line tool. No
binaries are distributed at the moment, so it must be built from source:

- Install Rust and Cargo if you don't already have them - see the official documentation
  at https://www.rust-lang.org/tools/install to install `rustup` and `cargo` and set up a
  toolchain
- Clone this repository
- Inside the root project directory, run `cargo build --release`
- Suggested - copy the binary created in `./target/release` to somewhere you'd like to keep
  binary tools, and add that location to your path

### Details

- Input files may be in JPEG, PNG, GIF or BMP format, while the output will always be a JPEG
  file named "stitchy.jpg". Operates entirely within the current directory.
- This project has currently only been tested (minimally) on 64-bit Windows using nightly Rust
  and the GNU toolchain, i.e. `nightly-x86_64-pc-windows-gnu`. Testing the MSVC toolchain, as
  well as the GNU toolchain on Linux, is likely to come soon.

### Future Plans

Here are some features I would like to add in the near future:

- Support for a version flag, `--version`, which returns the current version
- Support for horizontal-only and vertical-only arrangements, via `--horizontal`, `-h`,
  `--vertical` or `-v` flags
- Scaling to keep the output to sensible sizes, possibly using a flag like `--maxdim=n`
  to specify a maximum pixel size in either dimension
- Generate output files using incrementing suffixes where an output file already exists,
  i.e. "stitch_1.jpg", "stitch_2.jpg" etc. if "stitch.jpg" already exists
