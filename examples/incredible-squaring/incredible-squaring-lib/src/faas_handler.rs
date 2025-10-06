//! FaaS-compatible job handler that can be compiled to WASM
//!
//! This module provides a simple, synchronous interface for FaaS execution.
//! It's designed to be compiled to WASM and loaded by FaaS runtimes.

use serde::{Deserialize, Serialize};

/// Serializable input for FaaS execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FaasInput {
    pub job_id: u32,
    pub x: u64,
}

/// Serializable output from FaaS execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FaasOutput {
    pub result: u64,
}

/// The actual job logic - this is what runs in FaaS
///
/// This function is:
/// - Synchronous (no async in WASM yet)
/// - Simple types (no extractors)
/// - Self-contained (no external dependencies)
///
/// In production, this would be the core computation extracted from your blueprint job.
pub fn execute_square(x: u64) -> u64 {
    x * x
}

/// WASM-compatible entry point
///
/// This function can be called from WASM runtime with byte arrays.
#[no_mangle]
pub extern "C" fn faas_execute(input_ptr: *const u8, input_len: usize) -> *mut u8 {
    // Safety: This is the WASM boundary, caller must provide valid pointer
    let input_bytes = unsafe { std::slice::from_raw_parts(input_ptr, input_len) };

    // Deserialize input
    let input: FaasInput = match serde_json::from_slice(input_bytes) {
        Ok(input) => input,
        Err(_) => return std::ptr::null_mut(),
    };

    // Execute job logic
    let result = execute_square(input.x);

    // Serialize output
    let output = FaasOutput { result };
    let output_bytes = match serde_json::to_vec(&output) {
        Ok(bytes) => bytes,
        Err(_) => return std::ptr::null_mut(),
    };

    // Allocate and return pointer (WASM runtime will read this)
    let ptr = output_bytes.as_ptr() as *mut u8;
    std::mem::forget(output_bytes);
    ptr
}

/// Simple Rust-callable version for testing
pub fn handle_request(input: FaasInput) -> FaasOutput {
    FaasOutput {
        result: execute_square(input.x),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_execute_square() {
        assert_eq!(execute_square(5), 25);
        assert_eq!(execute_square(10), 100);
    }

    #[test]
    fn test_handle_request() {
        let input = FaasInput { job_id: 1, x: 7 };
        let output = handle_request(input);
        assert_eq!(output.result, 49);
    }
}
