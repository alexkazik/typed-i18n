use crate::attribute::parser::Parser;
use crate::attribute::Parameters;
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
            parser.finish(diagnostic);
            Some(Parameters {
                span,
                filename,
                separator,
            })
        } else {
            None
        }
    }
}
