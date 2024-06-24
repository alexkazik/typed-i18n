use crate::diagnostic::Diagnostic;
use crate::languages::Languages;
use crate::messages::message::Message;
use crate::messages::message_line::MessageLine;
use crate::messages::messages::Messages;
use crate::messages::param_type::ParamType;
use crate::messages::piece::Piece;
use indexmap::IndexMap;
use proc_macro2::{Ident, Span};
use std::borrow::Cow;
use std::collections::HashSet;
use std::str::from_utf8_unchecked;

pub(crate) struct RawMessages<'a>(
    #[allow(clippy::type_complexity)]
    pub(crate)  IndexMap<Cow<'a, str>, (Vec<Cow<'a, str>>, IndexMap<Cow<'a, str>, Cow<'a, str>>)>,
);

impl<'a> RawMessages<'a> {
    pub(crate) fn parse_values<'b: 'a, D: Diagnostic>(
        diagnostic: &mut D,
        span: Span,
        messages: RawMessages<'b>,
        languages: &Languages,
    ) -> Messages<'b> {
        let inner = messages
            .0
            .into_iter()
            .filter_map(|(k, (path, v))| {
                Self::parse_message(diagnostic, span, languages, k, path, v)
            })
            .collect::<IndexMap<_, _>>();
        if inner.is_empty() {
            diagnostic.emit_error(span, "no messages found");
            diagnostic.should_abort_if_dirty();
        }
        Messages::new(inner)
    }

    fn parse_message<'b, D: Diagnostic>(
        diagnostic: &mut D,
        span: Span,
        languages: &Languages,
        k: Cow<'b, str>,
        path: Vec<Cow<'b, str>>,
        v: IndexMap<Cow<'b, str>, Cow<'b, str>>,
    ) -> Option<(Cow<'b, str>, Message<'b>)> {
        let mut v_new = IndexMap::new();
        let mut params = Vec::new();
        for (lang, msg) in v {
            if !languages.iter().any(|l| l.name == lang) {
                diagnostic.emit_error(span, format!("language {lang} key {k} is not known"));
                continue;
            }
            let msg_line = MessageLine::build(msg, |msg| {
                let pieces = Self::parse_value(diagnostic, span, &k, &lang, msg);
                for p in &pieces {
                    match p {
                        Piece::Text(_) => {}
                        Piece::Param(p_name, p_type) => {
                            let p_name = *p_name;
                            let p_type = *p_type;
                            if let Some((_, old_p_type)) = params.iter().find(|(x, _)| *x == p_name)
                            {
                                if p_type != *old_p_type {
                                    diagnostic.emit_error(
                                            span,
                                            format!("mismatching types for parameter {p_name} in key {lang}")
                                        );
                                }
                            } else {
                                params.push((p_name.to_string(), p_type));
                            }
                        }
                    }
                }
                pieces
            });

            v_new.insert(lang, msg_line);
        }
        if v_new.is_empty() {
            diagnostic.emit_error(span, format!("key {k} has no values"));
            None
        } else {
            Some((
                k,
                Message {
                    path,
                    params,
                    message_lines: v_new,
                },
            ))
        }
    }

    fn parse_value<'b, D: Diagnostic>(
        diagnostic: &mut D,
        span: Span,
        k: &str,
        lang: &str,
        msg: &'b str,
    ) -> Vec<Piece<'b>> {
        let mut r = Vec::new();
        let mut t_params = HashSet::new();
        let msg = msg.as_bytes();
        let mut pos = 0;
        let mut start = 0;
        let mut in_param = false;
        let mut param_type = ParamType::Str;
        let mut has_error = false;

        while pos < msg.len() {
            if pos + 1 < msg.len() && (msg[pos] == b'%' || msg[pos] == b'*') && msg[pos + 1] == b'{'
            {
                if in_param {
                    has_error = true;
                    pos += 2;
                } else {
                    if pos > start {
                        // this is safe because it's only cut on ascii chars
                        r.push(Piece::Text(unsafe {
                            from_utf8_unchecked(&msg[start..pos])
                        }));
                    }
                    in_param = true;
                    param_type = if msg[pos] == b'%' {
                        ParamType::Str
                    } else {
                        ParamType::Typed
                    };
                    pos += 2;
                    start = pos;
                }
            } else if msg[pos] == b'}' {
                if !in_param || pos == start {
                    has_error = true;
                    in_param = false;
                    pos += 1;
                } else {
                    // this is safe because it's only cut on ascii chars
                    let p_name = unsafe { from_utf8_unchecked(&msg[start..pos]) };
                    if syn::parse_str::<Ident>(p_name).is_err() {
                        diagnostic.emit_error(
                            span,
                            format!(r#"invalid parameter name "{p_name}" in {k}.{lang}"#),
                        );
                    } else if !t_params.insert(p_name) {
                        diagnostic.emit_error(
                            span,  format!(
                            r#"duplicate use of a typed parameter: "{p_name}" in key {k}.{lang}"#
                        ));
                    } else {
                        r.push(Piece::Param(p_name, param_type));
                    }
                    in_param = false;
                    pos += 1;
                    start = pos;
                }
            } else {
                pos += 1;
            }
        }

        if in_param {
            has_error = true;
        } else if pos > start {
            // this is safe because it's only cut on ascii chars
            r.push(Piece::Text(unsafe {
                from_utf8_unchecked(&msg[start..pos])
            }));
        }

        if has_error {
            diagnostic.emit_error(span, format!("parse error in {k}.{lang}"));
        }

        r
    }
}
