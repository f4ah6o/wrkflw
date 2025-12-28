import * as vscode from "vscode";
import type {
  LanguageClient,
  LanguageClientOptions
} from "vscode-languageclient/browser";
import {
  log,
  LogLevel
} from "vscode-languageclient/browser";

let client: LanguageClient | null = null;

export function activate(context: vscode.ExtensionContext) {
  log(
    "WRKFLW LSP extension activating (Desktop)",
    LogLevel.Info,
    true
  );

  // Server is running in a WebWorker
  const serverModule = vscode.Uri.joinPath(
    context.extensionUri,
    "server",
    "out",
    "server.js"
  );

  const worker = new Worker(serverModule.toString(true));

  const clientOptions: LanguageClientOptions = {
    documentSelector: [
      { scheme: "file", pattern: "**/.github/workflows/*.yml" },
      { scheme: "file", pattern: "**/.github/workflows/*.yaml" }
    ],
    synchronize: {
      configurationSection: "wrkflw"
    },
    diagnosticCollectionName: "wrkflw",
    outputChannel: vscode.window.createOutputChannel("WRKFLW LSP")
  };

  // Import LanguageClient dynamically
  import("vscode-languageclient/browser").then(({ LanguageClient }) => {
    client = new LanguageClient(
      "wrkflw-lsp",
      "WRKFLW LSP Server",
      () => worker,
      clientOptions
    );

    client
      .start()
      .then(() => {
        log(
          "WRKFLW LSP client started",
          LogLevel.Info,
          true
        );
      })
      .catch((err) => {
        console.error("Failed to start LSP server:", err);
        void vscode.window.showErrorMessage(
          `Failed to start WRKFLW LSP: ${err}`
        );
      });
  });

  // Register validate command
  const validateCommand = vscode.commands.registerCommand(
    "wrkflw.validateWorkflow",
    async () => {
      const editor = vscode.window.activeTextEditor;
      if (!editor) {
        void vscode.window.showWarningMessage("No active editor");
        return;
      }

      const doc = editor.document;
      if (!doc.fileName.includes(".github/workflows")) {
        void vscode.window.showWarningMessage(
          "Not a GitHub Actions workflow file"
        );
        return;
      }

      await vscode.commands.executeCommand("workbench.action.diagnostic.focus");
    }
  );

  context.subscriptions.push(validateCommand);
}

export function deactivate(): Thenable<void> | undefined {
  if (!client) {
    return undefined;
  }
  return client.stop();
}
