import type {
  LanguageClientOptions,
  ServerOptions
} from "vscode-languageclient";

export interface ClientConfig {
  serverPath: string;
  documentSelector: string[];
}

export function getClientOptions(
  documentSelector: string[] = [
    { scheme: "file", pattern: "**/.github/workflows/*.yml" },
    { scheme: "file", pattern: "**/.github/workflows/*.yaml" }
  ]
): LanguageClientOptions {
  return {
    documentSelector,
    synchronize: {
      configurationSection: "wrkflw"
    },
    diagnosticCollectionName: "wrkflw"
  };
}

export function getServerOptions(serverPath: string): ServerOptions {
  return {
    command: serverPath,
    args: []
  };
}
