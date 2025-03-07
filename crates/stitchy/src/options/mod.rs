
#[cfg(test)]
mod tests;

mod v1;
mod v2;
mod v3;
mod v4;

pub(crate) use v1::Opt as OptV1;
pub(crate) use v2::Opt as OptV2;
pub(crate) use v3::Opt as OptV3;
pub use v4::Opt;

#[cfg(test)]
pub(crate) use v4::DEFAULT_QUALITY;
