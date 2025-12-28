const HOVER_INFO = {
    // Workflow keys
    name: `**Workflow name**

The name of the workflow. If omitted, GitHub uses the workflow file path.`,
    on: `**Trigger events**

The events that trigger the workflow. Can be a single event, array of events, or a map of event configurations.`,
    permissions: `**Permissions**

Set default permissions for all jobs in the workflow.`,
    env: `**Environment variables**

Environment variables available to all jobs in the workflow.`,
    defaults: `**Default settings**

Default settings for all jobs in the workflow.`,
    concurrency: `**Concurrency**

Ensures that only a single workflow run is active at a time.`,
    jobs: `**Jobs**

A map of jobs to run in the workflow.`,
    // Job keys
    "runs-on": `**Runner type**

The type of machine to run the job on. Can be a GitHub-hosted runner or a self-hosted runner.`,
    needs: `**Job dependencies**

Identifies any jobs that must complete successfully before this job will run.`,
    steps: `**Steps**

A sequence of tasks to execute in the job.`,
    strategy: `**Strategy**

A matrix strategy that creates multiple jobs.`,
    "timeout-minutes": `**Timeout**

The maximum number of minutes to let a job run before GitHub automatically cancels it.`,
    outputs: `**Outputs**

A map of outputs for a job.`,
    "continue-on-error": `**Continue on error**

Prevents a job from failing when a step fails.`,
    container: `**Container**

A container to run all steps in the job.`,
    services: `**Services**

Service containers to host with the job.`,
    uses: `**Reusable workflow**

Specifies a reusable workflow to run as a job.`,
    secrets: `**Secrets**

Secrets to pass to a reusable workflow.`,
    with: `**Inputs**

Inputs to pass to a reusable workflow or action.`,
    // Step keys
    run: `**Shell command**

Runs command-line programs using the operating system's shell.`,
    shell: `**Shell type**

The shell to use for the run command.`,
    "working-directory": `**Working directory**

The working directory to run the command in.`,
    // Trigger events
    push: `**Push event**

Runs your workflow when you push a commit or tag.`,
    pull_request: `**Pull request event**

Runs your workflow when activity on a pull request occurs.`,
    workflow_dispatch: `**Manual trigger**

Enables you to manually run a workflow from the Actions tab.`,
    schedule: `**Scheduled trigger**

Runs your workflow on a schedule using cron syntax.`,
    release: `**Release event**

Runs your workflow when a release is created or modified.`,
    // Runners
    "ubuntu-latest": `**Ubuntu Latest**

Latest Ubuntu runner (currently 24.04).`,
    "macos-latest": `**macOS Latest**

Latest macOS runner (currently macos-15).`,
    "windows-latest": `**Windows Latest**

Latest Windows runner (currently 2022).`,
    // Actions
    "actions/checkout": `**Checkout action**

Checks out your repository onto the runner, allowing you to run scripts or other actions against your code.`,
    "actions/setup-node": `**Setup Node.js**

Downloads and sets up a Node.js version.`,
    "actions/setup-python": `**Setup Python**

Downloads and sets up a Python version.`,
    "actions/cache": `**Cache action**

Caches dependencies and build outputs to improve workflow execution time.`
};
export function getHoverInfo(content, line, character) {
    const lines = content.split("\n");
    const lineText = lines[line];
    if (!lineText) {
        return null;
    }
    const word = findWordAtPosition(lineText, character);
    if (!word) {
        return null;
    }
    const hoverText = HOVER_INFO[word];
    if (!hoverText) {
        return null;
    }
    return {
        contents: {
            kind: "markdown",
            value: hoverText
        }
    };
}
function findWordAtPosition(line, charIdx) {
    if (charIdx >= line.length) {
        return null;
    }
    let start = charIdx;
    let end = charIdx;
    const isWordChar = (c) => /[a-zA-Z0-9\-_./]/.test(c);
    // Find word start
    while (start > 0 && isWordChar(line[start - 1])) {
        start--;
    }
    // Find word end
    while (end < line.length && isWordChar(line[end])) {
        end++;
    }
    if (start === end) {
        return null;
    }
    return line.slice(start, end);
}
//# sourceMappingURL=hover.js.map