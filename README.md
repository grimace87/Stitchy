
# Stitchy

Stitchy is an ecosystem of tools for joining multiple images together with good performance and a simple
interface. The core component is the `stitchy-core` crate written in the Rust language, while interface
applications are also available for the command line and as an Android app.

Configuration options include:
- Output format, supporting PNG, JPEG, WebP, and more
- Alignment of input images in the output
- Output image size limits

More options are available, depending on the component.

## Components

- [Stitchy Core](https://github.com/grimace87/Stitchy/tree/master/crates/stitchy-core)
- [Stitchy CLI](https://github.com/grimace87/Stitchy/tree/master/crates/stitchy)
- [Stitchy Mobile](https://play.google.com/store/apps/details?id=com.shininggrimace.stitchy)

## Examples

Images of the same size will stitch together neatly:

| Input files                         |                                     | Output                                            |
|-------------------------------------|-------------------------------------|---------------------------------------------------|
| ![Sample 1](./images/demo/cat1.jpg) | ![Sample 2](./images/demo/cat2.jpg) | ![Sample Output](./images/demo/stitched-cats.jpg) |

If the images are more irregular in shape, the tool will attempt to arrange them
as neatly as possible, and scale some images in the process:

| Input files | | |                                      | Output |
| --- | --- | --- |--------------------------------------| --- |
| ![Sample 1](./images/demo/Tree1.jpg) | ![Sample 2](./images/demo/Tree2.jpg) | ![Sample 3](./images/demo/Tree3.jpg) | ![Sample 4](./images/demo/Tree4.jpg) | ![Sample Output](./images/demo/TreeStitch.jpg) |
