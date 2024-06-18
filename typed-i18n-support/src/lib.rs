// #![warn(missing_docs)]
// enable pedantic group but not all
#![warn(clippy::pedantic)]
#![allow(clippy::too_many_lines)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::doc_markdown)]
#![allow(clippy::missing_errors_doc)]

//! Support crate for the derive macro for [typed-i18n](https://docs.rs/typed-i18n/latest/typed-i18n/).

pub mod attribute;
pub mod diagnostic;
pub(crate) mod generator;
pub mod languages;
pub mod messages;
