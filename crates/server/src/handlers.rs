#![allow(clippy::unnecessary_wraps)]
#![allow(clippy::needless_pass_by_value)]

use lsp_server::ResponseError;
use lsp_types::notification::PublishDiagnostics;
use lsp_types::{
    DocumentDiagnosticReport, DocumentDiagnosticReportResult, PublishDiagnosticsParams,
};

use analysis::{get_diagnostics, get_semantic_tokens};

use crate::server::{Context, Server};

pub(crate) fn handle_document_diagnostic_request(
    req: &lsp_types::DocumentDiagnosticParams,
    _lsp: &Server,
    ctx: &Context,
) -> Result<DocumentDiagnosticReportResult, ResponseError> {
    let path = req.text_document.uri.path().to_string();
    let doc = ctx
        .get_document(&path)
        .ok_or_else(|| doc_not_found_error(&path))?;

    let diagnostics = get_diagnostics(doc);
    Ok(DocumentDiagnosticReportResult::Report(
        DocumentDiagnosticReport::Full(lsp_types::RelatedFullDocumentDiagnosticReport {
            related_documents: None,
            full_document_diagnostic_report: lsp_types::FullDocumentDiagnosticReport {
                result_id: None,
                items: diagnostics,
            },
        }),
    ))
}

pub(crate) fn handle_did_open_text_document_params(
    params: lsp_types::DidOpenTextDocumentParams,
    lsp: &Server,
    ctx: &mut Context,
) {
    ctx.add_document(
        params.text_document.uri.path().to_string(),
        params.text_document.text,
    );

    let doc = ctx.get_document(params.text_document.uri.path()).unwrap();

    let diagnostics = get_diagnostics(doc);
    let params = PublishDiagnosticsParams {
        uri: params.text_document.uri,
        diagnostics,
        version: None,
    };
    lsp.send_notification::<PublishDiagnostics>(params);
}

pub(crate) fn handle_did_change_text_document_params(
    params: lsp_types::DidChangeTextDocumentParams,
    lsp: &Server,
    ctx: &mut Context,
) {
    ctx.update_document(
        params.text_document.uri.path(),
        params.content_changes.last().unwrap().text.clone(),
    );
    let doc = ctx.get_document(params.text_document.uri.path()).unwrap();
    let diagnostics = get_diagnostics(doc);
    let params = PublishDiagnosticsParams {
        uri: params.text_document.uri,
        diagnostics,
        version: None,
    };
    lsp.send_notification::<PublishDiagnostics>(params);
}

pub(crate) fn handle_did_close_text_document_params(
    params: lsp_types::DidCloseTextDocumentParams,
    _lsp: &Server,
    ctx: &mut Context,
) {
    ctx.remove_document(params.text_document.uri.path());
}

pub(crate) enum SyntaxTree {}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub(crate) struct SyntaxTreeParams {
    text_document: lsp_types::TextDocumentIdentifier,
}

impl lsp_types::request::Request for SyntaxTree {
    type Params = SyntaxTreeParams;
    type Result = String;
    const METHOD: &'static str = "camlot-analyzer/syntaxTree";
}

pub(crate) fn handle_syntax_tree_request(
    req: &SyntaxTreeParams,
    _lsp: &Server,
    ctx: &Context,
) -> Result<String, ResponseError> {
    let path = req.text_document.uri.path().to_string();
    let doc = ctx.get_document(&path).unwrap();
    Ok(doc.parsed().debug_tree())
}

pub(crate) fn handle_semantic_tokens_full_request(
    req: &lsp_types::SemanticTokensParams,
    _lsp: &Server,
    ctx: &Context,
) -> Result<Option<lsp_types::SemanticTokensResult>, ResponseError> {
    let path = req.text_document.uri.path().to_string();
    let doc = ctx.get_document(&path).unwrap();

    let tokens = get_semantic_tokens(doc);

    Ok(Some(lsp_types::SemanticTokensResult::Tokens(
        lsp_types::SemanticTokens {
            result_id: None,
            data: tokens,
        },
    )))
}

fn doc_not_found_error(path: &str) -> ResponseError {
    ResponseError {
        code: 0,
        message: format!("Document not found: {path}"),
        data: None,
    }
}
