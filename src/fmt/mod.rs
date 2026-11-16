//! Implementations for supported configuration formats.

pub mod cbr;
#[cfg(feature = "json")]
#[cfg_attr(docsrs, doc(cfg(feature = "json")))]
pub mod json;
#[cfg(feature = "ron")]
#[cfg_attr(docsrs, doc(cfg(feature = "ron")))]
pub mod ron;
#[cfg(feature = "toml")]
#[cfg_attr(docsrs, doc(cfg(feature = "toml")))]
pub mod toml;
#[cfg(feature = "yml")]
#[cfg_attr(docsrs, doc(cfg(feature = "yml")))]
pub mod yml;
