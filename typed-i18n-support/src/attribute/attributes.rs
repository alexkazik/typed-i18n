use crate::attribute::parser::Parser;
use crate::attribute::{Attributes, Builder, Parameters};
use crate::diagnostic::Diagnostic;
use proc_macro2::Span;
use syn::Attribute;

impl Attributes {
    pub fn parse<D: Diagnostic>(
        diagnostic: &mut D,
        span: Span,
        with_mixed_str: bool,
        attrs: Vec<Attribute>,
    ) -> Self {
        let mut parameters = None;
        let mut builders = Vec::new();
        for a in attrs {
            if let Some(mut parser) = Parser::parse(diagnostic, a) {
                if let Some(p) = Parameters::parse(diagnostic, &mut parser) {
                    if parameters.is_some() {
                        diagnostic.emit_error(p.span, "duplicate parameters");
                    }
                    parameters = Some(p);
                } else if let Some(builder) =
                    Builder::parse(diagnostic, &mut parser, with_mixed_str)
                {
                    builders.push(builder);
                } else {
                    diagnostic.emit_error(parser.span(), "unsupported path");
                }
            }
        }

        if parameters.is_none() {
            diagnostic.emit_error(span, "missing parameters");
        }
        if builders.is_empty() {
            diagnostic.emit_error(span, "no builders specified");
        }

        diagnostic.should_abort_if_dirty();

        Attributes {
            parameters: parameters.unwrap_or(Parameters {
                span,
                filename: "*.*".to_string(),
                separator: "_".to_string(),
            }),
            builders,
        }
    }
}
