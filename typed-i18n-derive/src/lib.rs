#![forbid(unsafe_code)]
#![forbid(missing_docs)]
#![warn(clippy::pedantic)]

//! Derive macro for [typed-i18n](https://docs.rs/typed-i18n/latest/typed-i18n/).

use proc_macro::TokenStream;
use proc_macro_error::{abort, proc_macro_error};
use std::env;
use std::path::PathBuf;
use syn::spanned::Spanned;
use syn::{parse_macro_input, DeriveInput};
use typed_i18n_support::attribute::Attributes;
use typed_i18n_support::diagnostic::ProcMacroError;
use typed_i18n_support::languages::RawLanguages;
use typed_i18n_support::messages::Messages;

/// Macro to convert a language file and an enum into a type safe i18n system.
#[proc_macro_error]
#[proc_macro_derive(TypedI18N, attributes(typed_i18n))]
pub fn typed_i18n(item: TokenStream) -> TokenStream {
    let diagnostic = &mut ProcMacroError;

    let input = parse_macro_input!(item as DeriveInput);
    let span = input.span();
    let with_mixed_str = cfg!(feature = "alloc");
    let attributes = Attributes::parse(diagnostic, span, with_mixed_str, input.attrs);

    let file_path = {
        let project_root = env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| ".".into());
        let mut path = PathBuf::from(project_root);
        path.push(&attributes.parameters.filename);
        path
    };

    let contents = std::fs::read_to_string(&file_path)
        .unwrap_or_else(|e| abort!(span, format!("Error reading file {file_path:?}: {e}")));

    let languages = RawLanguages::parse(diagnostic, span, input.data);

    let languages = languages.into(diagnostic, span);

    let messages = Messages::parse(
        diagnostic,
        span,
        &attributes.parameters,
        &languages,
        &contents,
    );

    let current_dir = env::current_dir().expect("Unable to get current directory");

    let relative_path = current_dir
        .join(file_path)
        .to_str()
        .expect("path contains invalid unicode")
        .to_string();

    attributes
        .generate(
            diagnostic,
            &input.vis,
            &input.ident,
            &relative_path,
            &languages,
            &messages,
        )
        .into()
}
