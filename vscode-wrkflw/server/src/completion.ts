import type { CompletionItem, CompletionItemKind } from "vscode-languageserver";

// GitHub Actions trigger events
const TRIGGER_EVENTS: Record<string, string> = {
  push: "Trigger on push to a branch",
  pull_request: "Trigger on pull request events",
  pull_request_target: "Trigger on pull request target (security context)",
  workflow_dispatch: "Manual workflow trigger",
  workflow_run: "Trigger on workflow run completion",
  repository_dispatch: "Trigger on repository event",
  release: "Trigger on release events",
  schedule: "Trigger on a schedule (cron)",
  status: "Trigger on commit status change",
  watch: "Trigger when star is added",
  fork: "Trigger when fork is created",
  delete: "Trigger when branch or tag is deleted",
  create: "Trigger when branch or tag is created",
  issues: "Trigger on issue events",
  issue_comment: "Trigger on issue comment events",
  label: "Trigger on label events",
  milestone: "Trigger on milestone events",
  discussion: "Trigger on discussion events",
  discussion_comment: "Trigger on discussion comment events",
  project: "Trigger on project events",
  project_card: "Trigger on project card events",
  project_column: "Trigger on project column events",
  package: "Trigger on package events",
  public: "Trigger when repository changes from private to public",
  page_build: "Trigger on GitHub Pages build",
  gollum: "Trigger on wiki page update",
  registry_package: "Trigger on registry package events",
  deployment: "Trigger on deployment events",
  deployment_status: "Trigger on deployment status change",
  check_run: "Trigger on check run events",
  check_suite: "Trigger on check suite events",
  content_reference: "Trigger on content reference events",
  merge_group: "Trigger on merge group events"
};

// Popular GitHub Actions
const POPULAR_ACTIONS: Record<string, string> = {
  "actions/checkout@v4": "Checkout repository code",
  "actions/setup-node@v4": "Setup Node.js environment",
  "actions/setup-python@v5": "Setup Python environment",
  "actions/setup-go@v5": "Setup Go environment",
  "actions/setup-java@v4": "Setup Java environment",
  "actions/setup-dotnet@v4": "Setup .NET environment",
  "actions/setup-rust@v1": "Setup Rust environment",
  "actions/cache@v4": "Cache dependencies",
  "actions/upload-artifact@v4": "Upload build artifacts",
  "actions/download-artifact@v4": "Download build artifacts",
  "actions/checkout": "Checkout repository code",
  "actions/setup-node": "Setup Node.js environment",
  "actions/setup-python": "Setup Python environment",
  "actions/stale": "Mark stale issues and PRs",
  "actions/create-release@v1": "Create GitHub release",
  "softprops/action-gh-release@v2": "Create GitHub release"
};

// GitHub-hosted runners
const RUNNERS: Record<string, string> = {
  "ubuntu-latest": "Latest Ubuntu runner",
  "ubuntu-24.04": "Ubuntu 24.04 runner",
  "ubuntu-22.04": "Ubuntu 22.04 runner",
  "ubuntu-20.04": "Ubuntu 20.04 runner",
  "macos-latest": "Latest macOS runner",
  "macos-15": "macOS 15 (Sequoia) runner",
  "macos-14": "macOS 14 (Sonoma) runner",
  "macos-13": "macOS 13 (Ventura) runner",
  "macos-12": "macOS 12 (Monterey) runner",
  "windows-latest": "Latest Windows runner",
  "windows-2022": "Windows 2022 runner",
  "windows-2019": "Windows 2019 runner"
};

// Workflow top-level keys
const WORKFLOW_KEYS: Record<string, string> = {
  name: "Workflow name",
  on: "Trigger events",
  permissions: "Set default permissions for jobs",
  env: "Environment variables for all jobs",
  defaults: "Default settings for all jobs",
  concurrency: "Concurrency group settings",
  jobs: "Jobs to run",
  "run-name": "Name for workflow runs"
};

// Job keys
const JOB_KEYS: Record<string, string> = {
  name: "Job name",
  "runs-on": "Runner type",
  needs: "Job dependencies",
  if: "Conditional execution",
  steps: "Job steps",
  strategy: "Matrix strategy",
  "timeout-minutes": "Timeout in minutes",
  env: "Job environment variables",
  defaults: "Default settings for steps",
  outputs: "Job outputs",
  permissions: "Job permissions",
  concurrency: "Job concurrency",
  "continue-on-error": "Continue on error",
  container: "Container to run in",
  services: "Service containers",
  uses: "Reusable workflow call",
  secrets: "Secrets to pass",
  with: "Inputs for reusable workflow"
};

// Step keys
const STEP_KEYS: Record<string, string> = {
  id: "Step ID",
  name: "Step name",
  uses: "Action to use",
  run: "Shell command to run",
  shell: "Shell type",
  with: "Action inputs",
  env: "Step environment variables",
  "continue-on-error": "Continue on error",
  "timeout-minutes": "Timeout in minutes",
  "working-directory": "Working directory"
};

const KIND_MAP: Record<string, CompletionItemKind> = {
  triggerEvents: 3, // CompletionItemKind.Event (deprecated, use Value)
  runners: 12, // CompletionItemKind.Value
  actions: 3, // CompletionItemKind.Function
  workflowKeys: 5, // CompletionItemKind.Field
  jobKeys: 5, // CompletionItemKind.Field
  stepKeys: 5 // CompletionItemKind.Field
} as const;

export type CompletionContextType =
  | "triggerEvents"
  | "runners"
  | "actions"
  | "workflowKeys"
  | "jobKeys"
  | "stepKeys"
  | "unknown";

export function getCompletionItems(
  content: string,
  line: number,
  character: number
): CompletionItem[] {
  const lines = content.split("\n");
  const currentLine = lines[line] ?? "";
  const currentTrimmed = currentLine.trim();
  const indent = currentLine.search(/\S/);

  // Get previous non-empty line
  let prevLine = "";
  for (let i = line - 1; i >= 0; i--) {
    const l = lines[i]?.trim();
    if (l && l !== "" && !l.startsWith("#")) {
      prevLine = l;
      break;
    }
  }

  const context = determineContext(prevLine, currentTrimmed, indent);

  if (context === "unknown") {
    return [];
  }

  const items = getItemsByContext(context);
  const kind = KIND_MAP[context] ?? 12; // Default to Value

  return Object.entries(items).map(([label, detail]) => ({
    label,
    kind,
    detail
  }));
}

function determineContext(
  prevLine: string,
  currentTrimmed: string,
  indent: number
): CompletionContextType {
  // Check if we're completing after "on:"
  if (prevLine === "on:" || prevLine.startsWith("on:")) {
    return "triggerEvents";
  }

  // Check if we're completing after "runs-on:"
  if (currentTrimmed === "runs-on:" || currentTrimmed.endsWith("runs-on:")) {
    return "runners";
  }

  // Check if we're completing after "uses:"
  if (currentTrimmed === "uses:" || currentTrimmed.endsWith("uses:")) {
    return "actions";
  }

  // Check indentation-based context
  if (currentTrimmed === "") {
    if (indent === 0) {
      return "workflowKeys";
    }
    if (indent === 2) {
      return "jobKeys";
    }
    if (indent === 4) {
      return "stepKeys";
    }
  }

  return "unknown";
}

function getItemsByContext(context: CompletionContextType): Record<string, string> {
  switch (context) {
    case "triggerEvents":
      return TRIGGER_EVENTS;
    case "runners":
      return RUNNERS;
    case "actions":
      return POPULAR_ACTIONS;
    case "workflowKeys":
      return WORKFLOW_KEYS;
    case "jobKeys":
      return JOB_KEYS;
    case "stepKeys":
      return STEP_KEYS;
    default:
      return {};
  }
}
