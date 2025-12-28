import initWasmModule, {
  expandMatrix,
  formatCombinationName,
  validateWorkflow
} from "../wasm/wrkflw_wasm.js";

let wasmInitialized = false;

export interface WasmModule {
  expandMatrix(matrix_json: string): unknown;
  formatCombinationName(job_name: string, combination_json: string): string;
  validateWorkflow(workflow_json: string): unknown;
}

/**
 * Initialize the WASM module
 * Must be called before using any WASM functions
 */
export async function initWasm(): Promise<void> {
  if (wasmInitialized) {
    return;
  }

  try {
    await initWasmModule();
    wasmInitialized = true;
  } catch (error) {
    console.error("Failed to initialize WASM module:", error);
    throw error;
  }
}

/**
 * Get the WASM module functions
 */
export async function getWasm(): Promise<WasmModule> {
  if (!wasmInitialized) {
    await initWasm();
  }

  return {
    expandMatrix,
    formatCombinationName,
    validateWorkflow
  };
}

/**
 * Check if WASM is initialized
 */
export function isWasmInitialized(): boolean {
  return wasmInitialized;
}

// Re-export functions for convenience
export { expandMatrix, formatCombinationName, validateWorkflow };
