
#[cfg(test)]
mod tests;

mod v1;
mod v2;

pub(crate) use v1::Opt as OptV1;
pub use v2::Opt;

#[cfg(test)]
pub(crate) use v2::DEFAULT_QUALITY;

pub fn deserialise_as_current(json: &str) -> Option<Opt> {

    // Try latest version
    if let Some(opt) = Opt::deserialise(json) {
        return Some(opt);
    }

    // Try v1
    if let Some(opt) = v1::Opt::deserialise(json) {
        return Some(opt.into());
    }

    None
}
