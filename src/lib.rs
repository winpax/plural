#![doc = include_str!("../README.md")]
#![warn(
    clippy::all,
    clippy::pedantic,
    rust_2018_idioms,
    rustdoc::all,
    rust_2024_compatibility,
    missing_docs
)]

mod form;
mod plural;

pub use form::PluralForm;

pub use plural::{Plural, Pluralize};
