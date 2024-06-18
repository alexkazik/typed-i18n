use crate::attribute::parser::Parser;
use crate::attribute::Builder;
use crate::diagnostic::Diagnostic;
use proc_macro2::{Ident, Span};
use syn::Type;

#[derive(Copy, Clone, Eq, PartialEq)]
pub(crate) enum BuilderVariant {
    StaticStr,
    MixedStr,
    Generic,
    Named,
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub(crate) enum StrConversion {
    Ref,
    AsRef,
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub(crate) enum InputVariant {
    None,
    Generic,
    Named,
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub(crate) enum InputConversion {
    Value,
    Into,
    Ref,
    AsRef,
}

impl Builder {
    pub(crate) fn parse<D: Diagnostic>(
        diagnostic: &mut D,
        parser: &mut Parser,
        with_mixed_str: bool,
    ) -> Option<Self> {
        if let Some((builder_span, builder_input)) = parser.remove("builder") {
            let builder = if builder_input == "static_str" {
                BuilderVariant::StaticStr
            } else if builder_input == "mixed_str" {
                if with_mixed_str {
                    BuilderVariant::MixedStr
                } else {
                    diagnostic.emit_error(
                        builder_span,
                        "builder mixed_str is only available with the feature alloc",
                    );
                    BuilderVariant::StaticStr
                }
            } else if builder_input == "_" {
                BuilderVariant::Generic
            } else {
                BuilderVariant::Named
            };
            let builder_type = match &builder {
                BuilderVariant::StaticStr | BuilderVariant::MixedStr => "String",
                BuilderVariant::Generic => "T",
                BuilderVariant::Named => &builder_input,
            };
            let builder_type = syn::parse_str::<Type>(builder_type).unwrap_or_else(|_| {
                diagnostic.emit_error(
                    parser.span(),
                    format!("invalid builder type: {builder_type}"),
                );
                syn::parse_str::<Type>("T").unwrap()
            });
            let prefix = parser.remove("prefix").map(|p| p.1);
            let str_conversion =
                parser
                    .remove("str_conversion")
                    .map_or(StrConversion::Ref, |(span, c)| match c.as_str() {
                        "ref" => StrConversion::Ref,
                        "as_ref" => StrConversion::AsRef,
                        _ => {
                            diagnostic.emit_error(span, format!("unsupported str conversion: {c}"));
                            StrConversion::Ref
                        }
                    });
            let mut input_ident = None;
            let mut input_variant = InputVariant::None;
            if let Some((span, s)) = parser.remove("input") {
                if s == "_" {
                    input_variant = InputVariant::Generic;
                } else if let Ok(i) = syn::parse_str(&s) {
                    input_ident = Some(i);
                    input_variant = InputVariant::Named;
                } else {
                    diagnostic.emit_error(span, format!("invalid input type: {s}"));
                }
            }
            let input_ident = input_ident.unwrap_or_else(|| Ident::new("I", Span::call_site()));
            let input_conversion = parser.remove("input_conversion").map_or(
                InputConversion::Into,
                |(span, c)| match c.as_str() {
                    "value" => InputConversion::Value,
                    "into" => InputConversion::Into,
                    "ref" => InputConversion::Ref,
                    "as_ref" => InputConversion::AsRef,
                    _ => {
                        diagnostic.emit_error(span, format!("unsupported input conversion: {c}"));
                        InputConversion::Into
                    }
                },
            );
            parser.finish(diagnostic);
            Some(Builder {
                span: parser.span(),
                builder_variant: builder,
                builder_type,
                prefix,
                str_conversion,
                input_ident,
                input_variant,
                input_conversion,
            })
        } else {
            None
        }
    }
}
