use crate::diagnostic::Diagnostic;
use crate::languages::Languages;
use crate::messages::messages::Messages;
use crate::messages::raw::RawMessages;
use indexmap::IndexMap;
use proc_macro2::{Ident, Span};
use std::borrow::Cow;
use std::mem;

impl<'a> RawMessages<'a> {
    pub(crate) fn parse_lrc<D: Diagnostic>(
        diagnostic: &mut D,
        span: Span,
        content: &'a str,
        languages: &Languages,
    ) -> Self {
        let mut current_msg = None;
        let mut current = IndexMap::new();
        let mut result = IndexMap::new();
        let mut key_pos = 0;
        for (pos, line) in content.lines().enumerate() {
            let line = line.trim();
            if line.is_empty() || line.starts_with(';') || line.starts_with('/') {
                // just skip this line
            } else if let Some(line) = line.strip_prefix('#') {
                if let Some(cm) = current_msg.take() {
                    if result
                        .insert(Cow::Borrowed(cm), (vec![], mem::take(&mut current)))
                        .is_some()
                    {
                        diagnostic
                            .emit_error(span, format!("duplicate key {cm} at line {key_pos}"));
                    }
                }
                key_pos = pos;
                let line = line.trim();
                if syn::parse_str::<Ident>(line).is_err() {
                    diagnostic.emit_error(span, format!("invalid key {line} at line {key_pos}"));
                }
                current_msg = Some(line);
            } else if current_msg.is_none() {
                diagnostic.emit_error(span, format!("value without key at line {pos}"));
            } else {
                let (a, b) = line.split_once(' ').unwrap_or((line, ""));
                let a = a.trim();
                if !languages.iter().any(|l| l.name == a) {
                    diagnostic
                        .emit_warning(span, format!("language {a} at line {pos} is not known"));
                } else if current
                    .insert(Cow::Borrowed(a), Cow::Borrowed(b.trim()))
                    .is_some()
                {
                    diagnostic.emit_error(span, format!("duplicate language {a} at line {pos}"));
                }
            }
        }

        if let Some(cm) = current_msg {
            if result
                .insert(Cow::Borrowed(cm), (vec![], mem::take(&mut current)))
                .is_some()
            {
                diagnostic.emit_error(span, format!("duplicate key {cm} at line {key_pos}"));
            }
        }

        RawMessages(result)
    }
}

impl<'a> Messages<'a> {
    #[must_use]
    pub fn to_lrc(&self) -> String {
        let mut output = String::new();
        for (k, m) in self {
            output.push('#');
            output.push_str(k);
            output.push('\n');
            for (l, ml) in &m.message_lines {
                output.push_str(l);
                output.push(' ');
                output.push_str(ml.borrow_line());
                output.push('\n');
            }
            output.push('\n');
        }
        output
    }
}
