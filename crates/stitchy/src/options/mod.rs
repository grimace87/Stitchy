
#[cfg(test)]
mod tests;

mod v1;
mod v2;

pub(crate) use v1::Opt as OptV1;
pub use v2::Opt;

#[cfg(test)]
pub(crate) use v2::DEFAULT_QUALITY;
