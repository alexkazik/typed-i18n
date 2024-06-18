use crate::diagnostic::Diagnostic;
use proc_macro2::Span;
use std::collections::BTreeMap;
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::{Attribute, Expr, ExprLit, Lit, Meta, MetaNameValue, Path, PathArguments, Token};

pub(crate) const ATTRIBUTE_NAME: &str = "typed_i18n";

pub(crate) struct Parser {
    span: Span,
    values: BTreeMap<String, (Span, String)>,
}

impl Parser {
    pub(crate) fn parse<D: Diagnostic>(diagnostic: &mut D, a: Attribute) -> Option<Parser> {
        if a.path().is_ident(ATTRIBUTE_NAME) {
            let span = a.span();
            if let Meta::List(meta_list) = a.meta {
                let nested = meta_list
                    .parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated)
                    .map_err(|e| {
                        diagnostic.emit_error(meta_list.span(), format!("meta parse error: {e}"));
                    })
                    .ok()?;
                let mut values = BTreeMap::new();
                for outer in nested {
                    if let Meta::NameValue(MetaNameValue {
                        path,
                        value:
                            Expr::Lit(ExprLit {
                                lit: Lit::Str(lit), ..
                            }),
                        ..
                    }) = outer
                    {
                        if let Some((name, span)) = path_to_name_span(diagnostic, path) {
                            if values.insert(name, (span, lit.value())).is_some() {
                                diagnostic.emit_error(span, "duplicate name");
                            }
                        }
                    } else {
                        diagnostic.emit_error(outer, "unsupported attribute type");
                    }
                }

                return Some(Parser { span, values });
            }
            diagnostic.emit_error(a.meta, "unsupported attribute type");
        }
        None
    }

    pub(crate) fn span(&self) -> Span {
        self.span
    }

    pub(crate) fn remove(&mut self, k: &str) -> Option<(Span, String)> {
        self.values.remove(k)
    }

    pub(crate) fn finish<D: Diagnostic>(&mut self, diagnostic: &mut D) {
        for (s, _) in self.values.values() {
            diagnostic.emit_error(*s, "unsupported path");
        }
    }
}

fn path_to_name_span<D: Diagnostic>(diagnostic: &mut D, path: Path) -> Option<(String, Span)> {
    if path.leading_colon.is_none() && path.segments.len() == 1 {
        let first = path.segments.first().unwrap();
        if matches!(first.arguments, PathArguments::None) {
            return Some((first.ident.to_string(), path.span()));
        }
    }
    diagnostic.emit_error(path, "unsupported path");
    None
}
