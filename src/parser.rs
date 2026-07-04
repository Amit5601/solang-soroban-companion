use anyhow::{Context, Result};
use std::fs;
use std::path::Path;

pub fn inspect_wasm_file(file_path: &Path) -> Result<()> {
    if !file_path.exists() {
        anyhow::bail!(
            "The file does not exist at the specified path: {:?}",
            file_path
        );
    }

    if file_path.extension().and_then(|ext| ext.to_str()) != Some("wasm") {
        anyhow::bail!("The specified file is not a valid .wasm file.");
    }

    let bytes = fs::read(file_path).context("Failed to read the raw bytes from the file")?;

    println!("✅ Success: Loaded file successfully!");
    println!("📦 File Size: {} bytes", bytes.len());

    if bytes.len() < 4 {
        anyhow::bail!("File is too small to be a valid WASM binary.");
    }

    let magic_number = &bytes[0..4];
    if magic_number != [0x00, 0x61, 0x73, 0x6D] {
        anyhow::bail!("Invalid WASM header detected: {:?}", magic_number);
    }

    println!("✨ Valid WebAssembly magic number detected.");
    println!("---");
    println!("🔍 Scanning contract for callable functions...");

    let functions = extract_functions(&bytes)?;
    for func in functions {
        println!("  ⚡ {}", func);
    }

    println!("---");
    Ok(())
}

// We extract this logic into a helper function so both `inspect` and `gen-test` can use it safely!
pub fn extract_functions(bytes: &[u8]) -> Result<Vec<String>> {
    let mut functions = Vec::new();

    for payload in wasmparser::Parser::new(0).parse_all(bytes) {
        if let Ok(wasmparser::Payload::ExportSection(reader)) = payload {
            for export in reader {
                if let Ok(e) = export
                    && matches!(e.kind, wasmparser::ExternalKind::Func)
                {
                    functions.push(e.name.to_string());
                }
            }
        }
    }

    Ok(functions)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_functions_empty_bytes() {
        let empty_bytes: Vec<u8> = vec![];
        let result = extract_functions(&empty_bytes);

        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 0);
    }
}
