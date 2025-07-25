
### 0.1.6 (July 26, 2025)

- Bump dependencies to pull in fixes and performance improvements (now requires Rust 1.80 or higher)

### 0.1.5 (March 8, 2025)

- Added types `RawBufferLocation` and `RawBufferProperties` for working with raw byte slices

### 0.1.4 (February 13, 2025)

- Support WebP (lossless only)
- Images with orientation metadata will now be correctly oriented (JPEG and WebP)
- Add 'fast mode' that uses nearest-neighbour filtering during image copy operations
- Fixed an issue that sometimes caused the stitch to fail

### 0.1.3 (January 12, 2025)

- Update image dependency to 0.25.5

### 0.1.2 (May 11, 2024)

- Fixed black lines sometimes appearing in the image due to imprecise downscaling
- Fixed stitch sometimes failing due to incorrect output bounds after imprecise downscaling
- Bumped image dependency to 0.24.9

### 0.1.1 (February 16, 2024)

- (BREAKING) Added traits `FileLocation` and `FileProperties`, where `ImageFiles` is generic over those
- Added types `OwnedRawFdLocation` and `OwnedRawFdProperties` for working with raw file descriptors (Unix systems only)
- (Credit: Friendly-Banana) Removed edge cases in grid layout to make it more intuitive

### 0.1.0 (April 29, 2023)

- Created this crate from parts of the original `stitchy` crate (version 0.1.4), allowing use outside the CLI program
- Cleaned up public API, deriving common traits for types, adding documentation
- Added builder patterns for source file selection and for stitching
- Added more ways to collect source files - arbitrary directory paths or individual files
- Fixed source files with uppercase letters in their extensions being ignored from source directories
- Added several tests confirming expected output dimensions given input dimensions
