
### Planned

- Fix occasional issue where it refuses to operate with images due to their dimensions
- Fix black border sometimes present even in relatively simple scenarios
- Manual control over grid layout

### 0.1.0

- Created this crate from parts of the original `stitchy` crate (version 0.1.4), allowing use outside the CLI program
- Cleaned up public API, deriving common traits for types, adding documentation
- Added builder patterns for source file selection and for stitching
- Added more ways to collect source files - arbitrary directory paths or individual files
- Fixed source files with uppercase letters in their extensions being ignored from source directories
- Added several tests confirming expected output dimensions given input dimensions
