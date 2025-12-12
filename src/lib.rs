#![deny(unsafe_code)] // At least for now 👻
//! # phenopacket-builder
//!
//! `phenopacket-builder` streamlines programmatic assembly of Phenopacket Schema building blocks.
//!
//! # Examples
//!
//! See the
//! [use cases](https://github.com/P2GX/phenopacket-builder/blob/master/tests/use_cases.rs)
//! for examples.

use phenopackets::schema::v2::core::OntologyClass;

mod v2;

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

/// A shortcut for creating an [`OntologyClass`] from its `id` and `label`.
///
/// # Example
///
/// ```
/// use phenopacket_builder::oc;
///
/// let seizure = oc("HP:0001250", "Seizure");
///
/// assert_eq!(&seizure.id, "HP:0001250");
/// assert_eq!(&seizure.label, "Seizure");
/// ```
pub fn oc(id: impl Into<String>, label: impl Into<String>) -> OntologyClass {
    OntologyClass {
        id: id.into(),
        label: label.into(),
    }
}
