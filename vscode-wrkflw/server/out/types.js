export function toDiagnosticSeverity(severity) {
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
export function toDiagnostic(issue) {
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
//# sourceMappingURL=types.js.map