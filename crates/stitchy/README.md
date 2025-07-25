
# Stitchy CLI

![example workflow](https://github.com/grimace87/Stitchy/actions/workflows/cargo.yml/badge.svg)
[![Crates.io](https://img.shields.io/crates/v/stitchy.svg)](https://crates.io/crates/stitchy)

CLI tool which wraps the features of the
[stitchy-core](https://github.com/grimace87/Stitchy/tree/master/crates/stitchy-core) crate, adding configuration
capabilities using command-line arguments and profile defaults, and adding file output.

See the [root project overview](https://github.com/grimace87/Stitchy) for an
overview of the Stitchy ecosystem.

## Minimum Supported Rist Version (MSRV)

Requires Rust version 1.80 or higher.

## Installing

- [Install the Rust programming language](https://www.rust-lang.org/tools/install) if you don't 
  already have it; this will include the `cargo` tool by default
- Run `cargo install stitchy`

That's it! The binary will be built from source and then become available in a command line.
If you installed Cargo with default settings, binaries will be in the `.cargo/bin` directory
inside your home directory, which will be available on your PATH.

### Windows

The installer may prompt you to install Visual Studio, since Rust requires a C/C++ compiler
and linker to be installed. The easiest way to set this up on Windows is to download the latest
version of Visual Studio, which can also be installed ahead of time, along with the components
for "Desktop Development with C++" which can be selected in the Visual Studio Installer.

For more information, see [MSVC prerequisites](https://rust-lang.github.io/rustup/installation/windows-msvc.html).

### NetBSD

If you are using NetBSD, `stitchy` is available from the official repositories. To install the
binary package, simply run:

```sh
pkgin install stitchy
```

## Running

The simplest case takes a given number of image files from the current directory and stitches
them using sensible configuration defaults:

`stitchy n`

where `n` is the number of images you would like to stitch together into one. The tool
will take the `n` most recent files and arrange them in a file named "stitch".

Various flags exist to adjust how source images are selected and how the output is generated.
Run `stitchy --help` to see a list of these flags.

## Configuration

For the full list of configuration options, run `stitchy --help`. Some options are:
- Set the desired output format using `--png`, `--jpeg`, `--gif`, `--bmp`, or `--webp`; for
  JPEG a quality option (0 to 100) can be passed like `--quality=___`, and for WebP only
  lossless is supported
- Set a limit on one output dimension using `--maxh=___` or `--maxw=___`, or both using
  `--maxd=___`

## Saving Defaults

Defaults can be saved to a file `.stitchyrc` in your home directory. Whenever you run `stitchy`,
these defaults are applied, unless you override them in the current command.

- Save default options using the flag `--setdefaults` and the options you want to save (if a
  number of images is provided it will not be saved in settings, ut the stitch will continue)
- Clear the defaults by running `stitchy --cleardefaults`
- Check the current defaults by running `stitchy --printdefaults`; this has the same effect as
  printing the contents of the `.stitchyrc` file to the terminal.

