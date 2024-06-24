use crate::attribute::builder::{BuilderVariant, InputConversion, InputVariant, StrConversion};
use proc_macro2::{Ident, Span};
use syn::Type;

pub(crate) mod attributes;
pub(crate) mod builder;
pub(crate) mod parameters;
pub(crate) mod parser;

pub struct Attributes {
    pub parameters: Parameters,
    pub builders: Vec<Builder>,
}

pub struct Builder {
    pub(crate) span: Span,
    #[allow(clippy::struct_field_names)] // because of a false positive
    pub(crate) builder_variant: BuilderVariant,
    #[allow(clippy::struct_field_names)] // because of a false positive
    pub(crate) builder_type: Type,
    pub(crate) prefix: Option<String>,
    pub(crate) str_conversion: StrConversion,
    pub(crate) input_ident: Ident,
    pub(crate) input_variant: InputVariant,
    pub(crate) input_conversion: InputConversion,
}

pub struct Parameters {
    pub span: Span,
    pub filename: String,
    /// Used for joining key parts in serde tree input.
    pub separator: String,
}

impl Parameters {
    #[must_use]
    pub fn run_new(filename: &str, separator: &str) -> Self {
        Self {
            span: Span::call_site(),
            filename: filename.to_string(),
            separator: separator.to_string(),
        }
    }
}
