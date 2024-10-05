import * as vscode from 'vscode';
import * as lc from 'vscode-languageclient';
import { Ctx } from './ctx';

// Opens the virtual file that will show the syntax tree
//
// The contents of the file come from the `TextDocumentContentProvider`
export function syntaxTree(ctx: Ctx) {
    const stcp = new SyntaxTreeContentProvider(ctx);

    ctx.pushCleanup(
        vscode.workspace.registerTextDocumentContentProvider(
            'camlot-analyzer',
            stcp,
        ),
    );

    vscode.workspace.onDidChangeTextDocument(
        (event: vscode.TextDocumentChangeEvent) => {
            const doc = event.document;
            if (doc.languageId !== 'camlot') {
                return;
            };
            afterLs(() => stcp.eventEmitter.fire(stcp.uri));
        },
        ctx.subscriptions,
    );

    vscode.window.onDidChangeActiveTextEditor(
        (editor: vscode.TextEditor | undefined) => {
            if (!editor || editor.document.languageId !== 'camlot') {
                return;
            }
            stcp.eventEmitter.fire(stcp.uri);
        },
        ctx.subscriptions,
    );

    return async () => {
        const editor = vscode.window.activeTextEditor;
        const rangeEnabled = !!(editor && !editor.selection.isEmpty);

        const uri = rangeEnabled
            ? vscode.Uri.parse(`${stcp.uri.toString()}?range=true`)
            : stcp.uri;

        const document = await vscode.workspace.openTextDocument(uri);

        stcp.eventEmitter.fire(uri);

        return vscode.window.showTextDocument(
            document,
            vscode.ViewColumn.Two,
            true,
        );
    };
}

// We need to order this after LS updates, but there's no API for that.
// Hence, good old setTimeout.
function afterLs(f: () => any) {
    setTimeout(f, 10);
}

interface SyntaxTreeParams {
    text_document: lc.TextDocumentIdentifier;
    range?: lc.Range;
}

export class SyntaxTreeContentProvider
    implements vscode.TextDocumentContentProvider {
    ctx: Ctx;
    uri = vscode.Uri.parse('camlot-analyzer://syntaxtree');
    eventEmitter = new vscode.EventEmitter<vscode.Uri>();
    syntaxTree: string = 'Not available';

    constructor(ctx: Ctx) {
        this.ctx = ctx;
    }

    provideTextDocumentContent(uri: vscode.Uri): vscode.ProviderResult<string> {
        const editor = vscode.window.activeTextEditor;
        if (editor === undefined) {
            return '';
        }

        let range: lc.Range | undefined;

        // When the range based query is enabled we take the range of the selection
        if (uri.query === 'range=true') {
            range = editor.selection.isEmpty
                ? undefined
                : this.ctx.client.code2ProtocolConverter.asRange(
                      editor.selection,
                  );
        }

        const request: SyntaxTreeParams = {
            text_document: { uri: editor.document.uri.toString() },
            range,
        };
        return this.ctx.client.sendRequest<string>(
            'camlot-analyzer/syntaxTree',
            request,
        );
    }

    get onDidChange(): vscode.Event<vscode.Uri> {
        return this.eventEmitter.event;
    }
}
