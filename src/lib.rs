#![deny(unsafe_code)] // At least for now 👻
//! # phenopacket-builder
//!
//! `phenopacket-builder` streamlines programmatic assembly of Phenopacket Schema building blocks.
//!
//! # Examples
//! TODO

mod builders;

#[derive(Copy, Clone, Debug, PartialEq, Hash, Eq)]
pub struct Set;
#[derive(Copy, Clone, Debug, Default, PartialEq, Hash, Eq)]
pub struct Unset;

pub trait Buildable {
    type Builder: Default;
    fn builder() -> Self::Builder {
        Self::Builder::default()
    }
}

pub trait Build<T> {
    fn build(self) -> T;
}

/// To allow submitting `T` where `Build<T>` is expected.
///
/// This is used across the builders.
///
/// # Example
///
/// ```
/// use phenopacket_builder::Build;
///
/// fn takes_build_val(val: impl Build<u8>) -> u8 {
///     val.build()
/// }
///
/// assert_eq!(takes_build_val(123), 123);
/// ```
impl<T, U> Build<U> for T
where
    T: Into<U>,
{
    fn build(self) -> U {
        self.into()
    }
}
