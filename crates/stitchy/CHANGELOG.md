
### 0.1.10 (July 26, 2025)

- Bump dependencies to pull in fixes and performance improvements (now requires Rust 1.80 or higher)

### 0.1.9 (February 13, 2025)

- Support WebP (lossless only)
- Images with orientation metadata will now be correctly oriented (JPEG and WebP)
- Output for `--help` now wraps text nicely for varying terminal size
- Add 'fast mode' that uses nearest-neighbour filtering during image copy operations
- Add 'small mode' that encodes the smallest file possible (at the expense of speed);
  applies only to PNG and GIF
- Add shorthand for --quality as -q
- Fixed an issue that sometimes caused the stitch to fail

### 0.1.8 (January 13, 2025)

- Updated image dependency to 0.25.5

### 0.1.7 (May 11, 2024)

- Specify input directory (`--input-dir` or `-i`) or output directory (`--output-dir` or `-o`); both
  default to current working directory

### 0.1.6 (February 16, 2024)

- Updated to support `stitchy-core` 0.1.1

### 0.1.5 (April 29, 2023)

- Moved much of the inner workings to the new `stitchy-core` crate, which is now a dependency
- Completely changed the flags used for controlling selection of source files in the current directory
- Added input/output file sizes and ratio to console output
- Fix: saving as non-JPEG when defaults specify JPEG with quality no longer fails
- Unit testing: several tests confirming expected output dimensions given input dimensions

### 0.1.4 (April 22, 2022)

- Updated to Rust 2021
- Updated dependencies (structopt to 0.3.26, image to 0.24.0)
- Print output file size in terminal output
- Improved handling of defaults

### 0.1.3 (October 14, 2020)

- More output options - PNG, GIF and BMP, with automatic matching source format if they're identical
- Can now operate on a single image, allowing downsizing and re-encoding into another format
- Can now select images based on ascending or descending alphabetical order
