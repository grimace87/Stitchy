
### Planned

- Publish inner workings as a separate library crate called "stitchy-core"
- More CLI output - display input/output file size ratio at least for single file transcoding
- Fix: cannot save as PNG if defaults contain JPEG and quality setting
- Fix: occasional issue where it refuses to operate with images due to their dimensions
- Fix: black border sometimes present even in relatively simple scenarios
- Unit testing: several tests confirming expected output dimensions given input dimensions

### 0.1.5 (Unreleased)

- Fix: source files with uppercase letters in their extensions are no longer ignored

### 0.1.4 (April 22, 2022)

- Updated to Rust 2021
- Updated dependencies (structopt to 0.3.26, image to 0.24.0)
- Print output file size in terminal output
- Added ability to save defaults that will apply automatically were not overridden

### 0.1.3 (October 14, 2020)

- More output options - PNG, GIF and BMP, with automatic matching source format if they're identical
- Can now operate on a single image, allowing downsizing and re-encoding into another format
- Can now select images based on ascending or descending alphabetical order
