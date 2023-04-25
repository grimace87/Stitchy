
# Stitchy Core

![example workflow](https://github.com/grimace87/Stitchy/actions/workflows/cargo.yml/badge.svg)
[![Crates.io](https://img.shields.io/crates/v/stitchy-core.svg)](https://crates.io/crates/stitchy-core)

Joins multiple existing image files into a single output. Builder patterns are provided to perform these operations.
Files can be added by individual paths, or in bulk from directories. This crate relies heavily on the `image` crates,
and the output is returned as an `image::DynamicImage`, which is re-exported from this crate for convenience.

Images of the same size will stitch together neatly:

| Input files                                | | | Output |
|--------------------------------------------| --- | --- | --- |
| ![Sample 1](../../images/demo/Screen1.jpg) | ![Sample 2](../../images/demo/Screen2.jpg) | ![Sample 3](../../images/demo/Screen3.jpg) | ![Sample Output](../../images/demo/ScreenStitch.jpg) |

If the images are more irregular in shape, the tool will attempt to arrange them
as neatly as possible, and scale some images in the process:

| Input files | | | | Output |
| --- | --- | --- | --- | --- |
| ![Sample 1](../../images/demo/Tree1.jpg) | ![Sample 2](../../images/demo/Tree2.jpg) | ![Sample 3](../../images/demo/Tree3.jpg) | ![Sample 3](../../images/demo/Tree4.jpg) | ![Sample Output](../../images/demo/TreeStitch.jpg) |

### Usage

See crate documentation for details on the API.
