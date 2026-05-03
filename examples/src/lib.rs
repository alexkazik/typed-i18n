#![forbid(unsafe_code)]
#![forbid(unused_crate_dependencies)]
#![warn(clippy::pedantic)]

mod cows;
mod html;

pub use crate::cows::Cows;
pub use crate::html::HtmlBuilder;
