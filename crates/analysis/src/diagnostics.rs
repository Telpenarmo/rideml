use line_index::TextRange;
use parser::SyntaxKind;

use crate::{offset_to_position, Document};

/// # Panics
///
/// Panics if parser produced different number of error nodes than messages.
#[must_use]
pub fn get_diagnostics(doc: &Document) -> Vec<lsp_types::Diagnostic> {
    let mut error_idx = 0;
    let parsed = doc.parsed();
    parsed
        .syntax()
        .descendants_with_tokens()
        .filter(|node| node.kind() == SyntaxKind::ERROR || node.kind() == SyntaxKind::LEXING_ERROR)
        .map(|node| {
            let msg = &parsed
                .errors
                .get(error_idx)
                .expect("Missing error message")
                .message;
            error_idx += 1;
            syntax_error_to_diagnostic(msg, node.text_range(), doc)
        })
        .collect()
}

fn syntax_error_to_diagnostic(
    message: &str,
    range: TextRange,
    doc: &Document,
) -> lsp_types::Diagnostic {
    let line_index = &doc.get_line_index();
    let start = offset_to_position(line_index, range.start().into());
    let end = offset_to_position(line_index, range.end().into());
    let range = lsp_types::Range::new(start, end);

    let mut diagnostic = lsp_types::Diagnostic::new_simple(range, message.to_string());
    diagnostic.source = Some("Camlot".into());
    diagnostic
}
