use crate::attribute::parser::Parser;
use crate::attribute::{Global, Parameters};
use crate::diagnostic::Diagnostic;
use unicode_ident::is_xid_continue;

impl Parameters {
    pub(crate) fn parse<D: Diagnostic>(diagnostic: &mut D, parser: &mut Parser) -> Option<Self> {
        if let Some((span, filename)) = parser.remove("filename") {
            let separator = parser
                .remove("separator")
                .map_or("_".to_string(), |(sp, sep)| {
                    if sep.chars().all(is_xid_continue) {
                        sep
                    } else {
                        diagnostic.emit_error(sp, "the separator contains invalid characters");
                        "_".to_string()
                    }
                });
            let global = parser.remove("global").and_then(|(sp, gl)| {
                if gl.as_str() == "atomic" {
                    Some(Global::Atomic)
                } else {
                    diagnostic.emit_error(sp, "the global is not known");
                    None
                }
            });
            parser.finish(diagnostic);
            Some(Parameters {
                span,
                filename,
                separator,
                global,
            })
        } else {
            None
        }
    }
}
