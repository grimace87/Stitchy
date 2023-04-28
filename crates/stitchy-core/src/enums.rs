
#[cfg(feature = "parser")]
use clap::ValueEnum;

#[cfg(feature = "parser")]
use serde::{Serialize, Deserialize};

/// Configure which end of the set of files to take from. The first file used will be the one
/// at the specified end, and then the next file in from the end, and so on.
/// The meaning of [TakeFrom::Start] or [TakeFrom::End] depends on the ordering specified with
/// [OrderBy].
///
/// With files named "a.jpg", "b.jpg", and "c.jpg", electing to stitch 2 files together with
/// [OrderBy::Alphabetic], then [TakeFrom::Start] will process files "a.jpg" then "b.jpg",
/// while [TakeFrom::End] will process files "c.jpg" then "b.jpg".
///
/// For ordering [OrderBy::Latest], the last-updated timestamps of the files determines the
/// order: [TakeFrom::Start] will begin with the most recent file first and working backwards,
/// while [TakeFrom::End] will take the oldest file and work forwards.
#[derive(PartialEq, Debug, Copy, Clone, Default)]
#[cfg_attr(feature = "parser", derive(ValueEnum, Serialize, Deserialize))]
pub enum TakeFrom {
    #[default]
    Start,
    End
}

/// Configure the order in which files are taken when selecting files from a set.
/// Specify which end of the list to take files from when stitching using [TakeFrom].
#[derive(PartialEq, Debug, Copy, Clone, Default)]
#[cfg_attr(feature = "parser", derive(ValueEnum, Serialize, Deserialize))]
pub enum OrderBy {
    #[default]
    Latest,
    Alphabetic
}
