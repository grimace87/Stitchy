
### Planned

- Handle rotation data from smartphone photos
- Manual control over grid layout
- Add support for WebP (and maybe more formats)
- Add 'fast mode' that uses nearest-neighbour filtering during image copy operations
- Multi-threading support

### Unreleased

- Fixed black lines sometimes appearing in the image due to imprecise downscaling
- Fixed stitch sometimes failing due to incorrect output bounds after imprecise downscaling

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
