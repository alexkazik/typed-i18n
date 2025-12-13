use crate::attribute::parser::Parser;
use crate::diagnostic::{Diagnostic, Simulated};
use convert_case::{Case, Casing};
use proc_macro2::Span;
use std::collections::HashSet;
use std::mem;
use std::slice::Iter;
use syn::spanned::Spanned;
use syn::{Data, Fields, Ident};

pub struct Languages(Vec<Language>);

impl Languages {
    pub fn iter(&self) -> Iter<'_, Language> {
        self.into_iter()
    }
}

#[allow(clippy::into_iter_without_iter)] // because of a false positive
impl<'a> IntoIterator for &'a Languages {
    type Item = &'a Language;
    type IntoIter = Iter<'a, Language>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

pub struct RawLanguages(pub Vec<Language>);

impl RawLanguages {
    pub fn iter(&self) -> Iter<'_, Language> {
        self.into_iter()
    }
}

impl<'a> IntoIterator for &'a RawLanguages {
    type Item = &'a Language;
    type IntoIter = Iter<'a, Language>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

pub struct Language {
    pub ident: Ident,
    pub name: String,
    pub fallback: Vec<String>,
    pub default: bool,
}

impl Language {
    /// Simplified function to create a new language.
    #[must_use]
    pub fn run_new(name: &str, fallback: &[&str]) -> Language {
        Language {
            ident: Ident::new(&name.to_case(Case::Pascal), Span::call_site()),
            name: name.to_string(),
            fallback: fallback.iter().map(ToString::to_string).collect(),
            default: false,
        }
    }
}

impl RawLanguages {
    pub fn parse<D: Diagnostic>(diagnostic: &mut D, span: Span, data: Data) -> Self {
        if let Data::Enum(data) = data {
            let mut values = Vec::new();

            for v in data.variants {
                let span = v.span();
                if !matches!(v.fields, Fields::Unit) {
                    diagnostic.emit_error(span, "only unit field items are allowed");
                }
                let mut has_attr = false;
                let mut name = None;
                let mut fallback = Vec::new();
                let mut default = false;

                for a in v.attrs {
                    let a_span = a.span();
                    if let Some(mut parser) = Parser::parse(diagnostic, a) {
                        if has_attr {
                            diagnostic.emit_error(a_span, "duplicate variant attribute");
                        } else {
                            has_attr = true;
                            name = parser.remove("name").map(|l| l.1);
                            if let Some((_, f)) = parser.remove("fallback") {
                                fallback = f
                                    .split(|s: char| s.is_whitespace() || s == ',')
                                    .filter(|s| !s.is_empty())
                                    .map(ToString::to_string)
                                    .collect();
                            }
                            if let Some((s, v)) = parser.remove("default") {
                                match v.as_str() {
                                    "true" => {
                                        default = true;
                                    }
                                    "false" => {}
                                    _ => diagnostic.emit_error(s, "unknown default value"),
                                }
                            }
                            parser.finish(diagnostic);
                        }
                    }
                }

                let name = name.unwrap_or_else(|| v.ident.to_string().to_case(Case::Snake));
                values.push(Language {
                    ident: v.ident,
                    name,
                    fallback,
                    default,
                });
            }

            if values.is_empty() {
                diagnostic.emit_error(span, "no variants found");
                diagnostic.should_abort_if_dirty();
            }

            RawLanguages(values)
        } else {
            diagnostic.emit_error(span, "only enums are supported");
            diagnostic.should_abort_if_dirty();

            RawLanguages(vec![])
        }
    }

    /// Simplified function to run the into conversion, will result in an error if any diagnostics are emitted.
    pub fn run_into(self) -> Result<Languages, Simulated> {
        Simulated::run(|diagnostic| self.into(diagnostic, Span::call_site()))
    }

    pub fn into<D: Diagnostic>(self, diagnostic: &mut D, span: Span) -> Languages {
        let mut languages = self.0;
        if languages.is_empty() {
            diagnostic.emit_error(span, "no languages found");
            diagnostic.should_abort_if_dirty();
        }
        let defaults = languages.iter().filter(|l| l.default).count();
        if defaults == 0 {
            if let Some(l) = languages.iter_mut().next() {
                l.default = true;
            }
        } else if defaults > 1 {
            diagnostic.emit_error(span, "found more than one default language");
        }
        let names = languages.iter().map(|v| v.name.clone()).collect::<Vec<_>>();
        let mut seen_languages = HashSet::new();
        for v in &mut languages {
            if !seen_languages.insert(&v.name) {
                diagnostic.emit_error(v.ident.span(), "language defined twice");
            }
            let v_fallback = mem::take(&mut v.fallback);
            if v_fallback.contains(&v.name) {
                diagnostic.emit_error(v.ident.span(), "fallback of itself");
            }
            let (l, r) = v_fallback
                .into_iter()
                .filter(|f| f != &v.name)
                .partition::<Vec<_>, _>(|f| names.contains(f));
            for r in r {
                diagnostic.emit_error(v.ident.span(), format!("unknown fallback: {r}"));
            }
            v.fallback.clear();
            v.fallback.push(v.name.clone());
            v.fallback.extend(l);
            for name in &names {
                if !v.fallback.contains(name) {
                    v.fallback.push(name.clone());
                }
            }
        }
        Languages(languages)
    }
}

#[cfg(test)]
mod tests {
    use crate::languages::{Language, RawLanguages};

    #[test]
    fn raw_language_iter() {
        assert_eq!(
            RawLanguages(vec![Language::run_new("en", &[])])
                .iter()
                .map(|l| l.name.as_str())
                .collect::<Vec<_>>(),
            vec!["en"]
        );
    }
}
