import type {
  CompletionParams,
  HoverParams,
  Diagnostic
} from "vscode-languageserver-protocol";
import {
  createConnection,
  BrowserMessageReader,
  BrowserMessageWriter,
  InitializeParams,
  InitializeResult,
  TextDocumentSyncKind,
  TextDocuments
} from "vscode-languageserver/browser";
import { TextDocument } from "vscode-languageserver-textdocument";
import { validateWorkflow } from "./diagnostics.js";
import { getCompletionItems } from "./completion.js";
import { getHoverInfo } from "./hover.js";

// WebWorker context
declare const self: DedicatedWorkerGlobalScope;

const messageReader = new BrowserMessageReader(self);
const messageWriter = new BrowserMessageWriter(self);

const connection = createConnection(messageReader, messageWriter);
const documents = new TextDocuments(TextDocument);

// Track open documents
const documentCache = new Map<string, string>();

connection.onInitialize((_params: InitializeParams): InitializeResult => {
  return {
    capabilities: {
      textDocumentSync: TextDocumentSyncKind.Full,
      completionProvider: {
        triggerCharacters: [" ", ":"],
        resolveProvider: false
      },
      hoverProvider: true
    }
  };
});

// Document lifecycle
documents.onDidOpen((event) => {
  documentCache.set(event.document.uri, event.document.getText());

  // Validate on open
  const validationResult = validateWorkflow(event.document.getText(), event.document.uri);
  if (validationResult.diagnostics.length > 0) {
    connection.sendDiagnostics({
      uri: event.document.uri,
      diagnostics: validationResult.diagnostics
    });
  }
});

documents.onDidChangeContent((event) => {
  documentCache.set(event.document.uri, event.document.getText());

  // Validate on change
  const validationResult = validateWorkflow(event.document.getText(), event.document.uri);
  connection.sendDiagnostics({
    uri: event.document.uri,
    diagnostics: validationResult.diagnostics
  });
});

documents.onDidClose((event) => {
  documentCache.delete(event.document.uri);
  connection.sendDiagnostics({
    uri: event.document.uri,
    diagnostics: []
  });
});

// Completion handler
connection.onCompletion((params: CompletionParams) => {
  const uri = params.textDocument.uri;
  const content = documentCache.get(uri);

  if (!content) {
    return [];
  }

  return getCompletionItems(content, params.position.line, params.position.character);
});

// Hover handler
connection.onHover((params: HoverParams) => {
  const uri = params.textDocument.uri;
  const content = documentCache.get(uri);

  if (!content) {
    return null;
  }

  return getHoverInfo(content, params.position.line, params.position.character);
});

// Start listening
documents.listen(connection);
connection.listen();
