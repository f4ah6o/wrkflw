import type { Diagnostic } from "vscode-languageserver";
import { parseDocument } from "yaml";

export interface ValidationResult {
  isValid: boolean;
  diagnostics: Diagnostic[];
}

export function validateWorkflow(content: string, uri: string): ValidationResult {
  const diagnostics: Diagnostic[] = [];

  // Check if file is in .github/workflows
  if (!uri.includes(".github/workflows")) {
    return { isValid: true, diagnostics: [] };
  }

  // Parse YAML
  const doc = parseDocument(content);

  if (doc.errors.length > 0) {
    for (const error of doc.errors) {
      const line = error.linePos?.[0]?.line ?? 0;
      const col = error.linePos?.[0]?.col ?? 0;
      diagnostics.push({
        severity: 1, // Error
        range: {
          start: { line: line - 1, character: col - 1 },
          end: { line: line - 1, character: col + 10 }
        },
        message: `YAML syntax error: ${error.message}`,
        source: "wrkflw"
      });
    }
  }

  if (doc.warnings.length > 0) {
    for (const warning of doc.warnings) {
      const line = warning.linePos?.[0]?.line ?? 0;
      const col = warning.linePos?.[0]?.col ?? 0;
      diagnostics.push({
        severity: 2, // Warning
        range: {
          start: { line: line - 1, character: col - 1 },
          end: { line: line - 1, character: col + 10 }
        },
        message: `YAML warning: ${warning.message}`,
        source: "wrkflw"
      });
    }
  }

  // Validate workflow structure
  if (doc.errors.length === 0) {
    const workflow = doc.toJS() as Record<string, unknown>;

    if (!workflow.name && !workflow.on) {
      diagnostics.push({
        severity: 2, // Warning
        range: {
          start: { line: 0, character: 0 },
          end: { line: 0, character: 10 }
        },
        message: "Workflow is missing 'name' and 'on' fields",
        source: "wrkflw"
      });
    }

    if (!workflow.jobs) {
      diagnostics.push({
        severity: 1, // Error
        range: {
          start: { line: 0, character: 0 },
          end: { line: 0, character: 10 }
        },
        message: "Workflow must define 'jobs' section",
        source: "wrkflw"
      });
    } else if (typeof workflow.jobs === "object" && Object.keys(workflow.jobs).length === 0) {
      diagnostics.push({
        severity: 1, // Error
        range: {
          start: { line: 0, character: 0 },
          end: { line: 0, character: 10 }
        },
        message: "Workflow must define at least one job",
        source: "wrkflw"
      });
    }

    // Validate trigger events
    if (workflow.on) {
      const triggers = Array.isArray(workflow.on) ? workflow.on : [workflow.on];
      const validTriggers = new Set([
        "push",
        "pull_request",
        "pull_request_target",
        "workflow_dispatch",
        "workflow_run",
        "repository_dispatch",
        "release",
        "schedule",
        "status",
        "watch",
        "fork",
        "delete",
        "create",
        "issues",
        "issue_comment",
        "label",
        "milestone",
        "discussion",
        "discussion_comment",
        "project",
        "project_card",
        "project_column",
        "package",
        "public",
        "page_build",
        "gollum",
        "registry_package",
        "deployment",
        "deployment_status",
        "check_run",
        "check_suite",
        "content_reference",
        "merge_group"
      ]);

      for (const trigger of triggers) {
        if (typeof trigger === "string" && !validTriggers.has(trigger)) {
          // Find the line with 'on:'
          const lines = content.split("\n");
          const onLine = lines.findIndex((l) => l.trim().startsWith("on:"));
          if (onLine !== -1) {
            diagnostics.push({
              severity: 2, // Warning
              range: {
                start: { line: onLine, character: 0 },
                end: { line: onLine, character: 10 }
              },
              message: `Unknown trigger event: ${trigger}`,
              source: "wrkflw"
            });
          }
        }
      }
    }
  }

  return {
    isValid: diagnostics.filter((d) => d.severity === 1).length === 0,
    diagnostics
  };
}
