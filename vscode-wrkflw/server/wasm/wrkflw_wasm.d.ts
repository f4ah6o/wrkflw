/* tslint:disable */
/* eslint-disable */

/**
 * Expand a matrix configuration into all possible combinations
 *
 * # Arguments
 * * `matrix_json` - JSON string of the matrix configuration
 *
 * # Returns
 * * JSON string of the expanded combinations
 *
 * # Example
 * ```javascript
 * const matrix = {
 *   parameters: {
 *     os: ["ubuntu-latest", "windows-latest"],
 *     node: [14, 16, 18]
 *   }
 * };
 * const combinations = expandMatrix(JSON.stringify(matrix));
 * ```
 */
export function expandMatrix(matrix_json: string): any;

/**
 * Format a matrix combination name
 *
 * # Arguments
 * * `job_name` - The base job name
 * * `combination_json` - JSON string of the combination values
 *
 * # Returns
 * * Formatted combination name
 */
export function formatCombinationName(job_name: string, combination_json: string): string;

/**
 * Initialize the panic hook for better error messages in the browser
 */
export function init_panic_hook(): void;

/**
 * Validate a workflow structure
 *
 * # Arguments
 * * `workflow_json` - JSON string of the workflow YAML content
 *
 * # Returns
 * * JSON string of validation results
 */
export function validateWorkflow(workflow_json: string): any;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly init_panic_hook: () => void;
  readonly expandMatrix: (a: number, b: number) => [number, number, number];
  readonly formatCombinationName: (a: number, b: number, c: number, d: number) => [number, number, number, number];
  readonly validateWorkflow: (a: number, b: number) => [number, number, number];
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
  readonly __wbindgen_free: (a: number, b: number, c: number) => void;
  readonly __wbindgen_externrefs: WebAssembly.Table;
  readonly __externref_table_dealloc: (a: number) => void;
  readonly __wbindgen_start: () => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;

/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {{ module: SyncInitInput }} module - Passing `SyncInitInput` directly is deprecated.
*
* @returns {InitOutput}
*/
export function initSync(module: { module: SyncInitInput } | SyncInitInput): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {{ module_or_path: InitInput | Promise<InitInput> }} module_or_path - Passing `InitInput` directly is deprecated.
*
* @returns {Promise<InitOutput>}
*/
export default function __wbg_init (module_or_path?: { module_or_path: InitInput | Promise<InitInput> } | InitInput | Promise<InitInput>): Promise<InitOutput>;
