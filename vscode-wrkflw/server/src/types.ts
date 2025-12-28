import type {
  Diagnostic,
  DiagnosticSeverity,
  CompletionItem,
  CompletionItemKind,
  Hover,
  Position
} from "vscode-languageserver";

export interface WorkflowValidationResult {
  isValid: boolean;
  issues: ValidationIssue[];
}

export interface ValidationIssue {
  message: string;
  severity: "error" | "warning" | "info";
  line?: number;
  column?: number;
  source?: string;
}

export interface CompletionContext {
  type:
    | "triggerEvents"
    | "runners"
    | "actions"
    | "workflowKeys"
    | "jobKeys"
    | "stepKeys"
    | "unknown";
  indent: number;
}

export function toDiagnosticSeverity(
  severity: ValidationIssue["severity"]
): DiagnosticSeverity {
  switch (severity) {
    case "error":
      return 1; // DiagnosticSeverity.Error
    case "warning":
      return 2; // DiagnosticSeverity.Warning
    case "info":
      return 3; // DiagnosticSeverity.Information
    default:
      return 1;
  }
}

export function toDiagnostic(issue: ValidationIssue): Diagnostic {
  return {
    severity: toDiagnosticSeverity(issue.severity),
    range: {
      start: { line: issue.line ?? 0, character: issue.column ?? 0 },
      end: { line: issue.line ?? 0, character: issue.column ?? 100 }
    },
    message: issue.message,
    source: issue.source ?? "wrkflw"
  };
}
