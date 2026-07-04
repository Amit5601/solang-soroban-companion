use crate::parser::extract_functions;
use anyhow::{Context, Result};
use std::fs;
use std::path::Path;

pub fn generate_test_boilerplate(file_path: &Path) -> Result<()> {
    if !file_path.exists() {
        anyhow::bail!(
            "The file does not exist at the specified path: {:?}",
            file_path
        );
    }

    let bytes = fs::read(file_path).context("Failed to read the raw bytes from the file")?;

    println!("🛠️ Generating tests for: {:?}", file_path);

    // Call our parser helper function
    let exported_functions = extract_functions(&bytes)?;

    let mut test_file_content = String::new();
    test_file_content.push_str("#![cfg(test)]\n\n");
    test_file_content.push_str("use soroban_sdk::{Env, BytesN, testutils::Address as _};\n\n");
    test_file_content.push_str("extern crate std;\n\n");

    // Explicitly include a placeholder for the WASM bytecode array so the test can register it
    test_file_content.push_str("// The compiled WASM bytecode of the contract\n");
    test_file_content.push_str("const WASM: &[u8] = include_bytes!(\"./dummy.wasm\");\n\n");

    for func_name in exported_functions {
        if func_name == "memory" {
            continue;
        }

        test_file_content.push_str("#[test]\n");
        test_file_content.push_str(&format!("fn test_{}() {{\n", func_name));
        test_file_content.push_str("    let env = Env::default();\n");
        test_file_content
            .push_str("    let contract_id = env.register_contract_wasm(None, WASM);\n");
        test_file_content.push_str("    \n");
        test_file_content.push_str(&format!(
            "    // TODO: Invoke contract function: {}\n",
            func_name
        ));
        test_file_content
            .push_str("    // let client = ContractClient::new(&env, &contract_id);\n");
        test_file_content.push_str("    \n");
        test_file_content.push_str("    assert!(true);\n");
        test_file_content.push_str("}\n\n");
    }

    fs::write("contract_test.rs", test_file_content)
        .context("Failed to save the generated test file to disk")?;

    println!("✅ Success: Created 'contract_test.rs'!");

    Ok(())
}
