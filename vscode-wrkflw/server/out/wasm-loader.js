import initWasmModule, { expandMatrix, formatCombinationName, validateWorkflow } from "../wasm/wrkflw_wasm.js";
let wasmInitialized = false;
/**
 * Initialize the WASM module
 * Must be called before using any WASM functions
 */
export async function initWasm() {
    if (wasmInitialized) {
        return;
    }
    try {
        await initWasmModule();
        wasmInitialized = true;
    }
    catch (error) {
        console.error("Failed to initialize WASM module:", error);
        throw error;
    }
}
/**
 * Get the WASM module functions
 */
export async function getWasm() {
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
export function isWasmInitialized() {
    return wasmInitialized;
}
// Re-export functions for convenience
export { expandMatrix, formatCombinationName, validateWorkflow };
//# sourceMappingURL=wasm-loader.js.map