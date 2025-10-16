//! Standalone FaaS handler binary
//!
//! This binary mimics how AWS Lambda works:
//! 1. Receives FaasPayload via stdin (or HTTP in real Lambda)
//! 2. Executes the actual compiled job logic
//! 3. Returns FaasResponse via stdout
//!
//! This is the ACTUAL code that would run in production FaaS!

use serde::{Deserialize, Serialize};
use std::io::{self, Read, Write};

#[derive(Debug, Serialize, Deserialize)]
struct FaasPayload {
    job_id: u32,
    #[serde(with = "serde_bytes")]
    args: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize)]
struct FaasResponse {
    #[serde(with = "serde_bytes")]
    result: Vec<u8>,
}

fn main() -> io::Result<()> {
    // Read input from stdin (Lambda-style)
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    // Deserialize payload
    let payload: FaasPayload = serde_json::from_str(&input)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

    // Extract input (u64 from little-endian bytes)
    let x = u64::from_le_bytes(
        payload.args[..8]
            .try_into()
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Invalid args length"))?
    );

    // ⚡ EXECUTE THE ACTUAL JOB LOGIC (compiled code!)
    let result = x * x;

    // Package response
    let response = FaasResponse {
        result: result.to_le_bytes().to_vec(),
    };

    // Write output to stdout (Lambda-style)
    let output = serde_json::to_string(&response)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

    io::stdout().write_all(output.as_bytes())?;
    io::stdout().flush()?;

    Ok(())
}
