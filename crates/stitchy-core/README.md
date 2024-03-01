
# Stitchy Core

![example workflow](https://github.com/grimace87/Stitchy/actions/workflows/cargo.yml/badge.svg)
[![Crates.io](https://img.shields.io/crates/v/stitchy-core.svg)](https://crates.io/crates/stitchy-core)

Joins multiple existing image files into a single output. Design features include:
- Builder structures for applying common usage
- Collecting source files individually by path, or in bulk from directories
- Re-exports from the `image` crate, on which this crate relies heavily
and the output is returned as an `image::DynamicImage`, which is re-exported from this crate for convenience.

See the [root project overview](https://github.com/grimace87/Stitchy) for an
overview of the Stitchy ecosystem.

## Typical Usage

To take the 3 most recently updated files in the current directory, ordering them
oldest to newest, and writing the output to the current directory, run:

```rust
use stitchy_core::{ImageFiles, FilePathWithMetadata, OrderBy, TakeFrom, Stitch, AlignmentMode, image::ImageOutputFormat};
use std::fs::File;
use std::path::PathBuf;

fn run_stitch() -> Result<(), String> {
    let number_of_files = 3;

    let image_contents = ImageFiles::<FilePathWithMetadata>::builder()
        .add_current_directory(vec![])?
        .build()?
        .sort_and_truncate_by(
            number_of_files,
            OrderBy::Latest,
            TakeFrom::Start,
            false
        )?
        .into_image_contents(true)?;

    let output = Stitch::builder()
        .images(image_contents)
        .alignment(AlignmentMode::Horizontal)
        .stitch()?;

    let mut file_path: PathBuf = std::env::current_dir().unwrap();
    file_path.push("stitch.png");
    let mut file_writer = File::create(file_path).unwrap();
    output.write_to(&mut file_writer, ImageOutputFormat::Png)
        .map_err(|_| "File metadata could not be read.".to_owned())?;
    Ok(())
}
```

See these examples for more complicated real-world usage:
- [Stitchy CLI - main.rs](https://github.com/grimace87/Stitchy/blob/master/crates/stitchy/src/main.rs)
- [Stitchy Mobile - stitch.rs](https://github.com/grimace87/StitchyMobile/blob/master/rust/src/stitch.rs)
