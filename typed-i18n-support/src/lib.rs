// #![warn(missing_docs)]
// enable pedantic group but not all
#![warn(clippy::pedantic)]
#![allow(
    clippy::doc_markdown,
    clippy::missing_errors_doc,
    clippy::missing_panics_doc,
    clippy::module_inception,
    clippy::module_name_repetitions,
    clippy::too_many_lines
)]

//! Support crate for the derive macro for [typed-i18n](https://docs.rs/typed-i18n/latest/typed-i18n/).

pub mod attribute;
pub mod diagnostic;
pub(crate) mod generator;
pub mod languages;
pub mod messages;
