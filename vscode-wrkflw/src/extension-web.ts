import * as vscode from "vscode";
import type {
  LanguageClient,
  LanguageClientOptions
} from "vscode-languageclient/browser";

let client: LanguageClient | null = null;

export function activate(context: vscode.ExtensionContext) {
  console.log("WRKFLW LSP extension activating (Web)");

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
    outputChannelName: "WRKFLW LSP"
  };

  // Import LanguageClient dynamically for web
  import("vscode-languageclient/browser").then(({ LanguageClient }) => {
    client = new LanguageClient(
      "wrkflw-lsp",
      "WRKFLW LSP Server",
      () => worker,
      clientOptions
    );

    client.start().catch((err) => {
      console.error("Failed to start LSP server:", err);
    });
  });

  // Register validate command
  const validateCommand = vscode.commands.registerCommand(
    "wrkflw.validateWorkflow",
    async () => {
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
