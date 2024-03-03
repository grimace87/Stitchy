
#[cfg(test)]
mod tests;

mod v1;
mod v2;
mod v3;

pub(crate) use v1::Opt as OptV1;
pub(crate) use v2::Opt as OptV2;
pub use v3::Opt;

#[cfg(test)]
pub(crate) use v3::DEFAULT_QUALITY;
