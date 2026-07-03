#![cfg(test)]

use soroban_sdk::{Env, BytesN, testutils::Address as _};

extern crate std;

// The compiled WASM bytecode of the contract
const WASM: &[u8] = include_bytes!("./dummy.wasm");

#[test]
fn test_test_function() {
    let env = Env::default();
    let contract_id = env.register_contract_wasm(None, WASM);
    
    // TODO: Invoke contract function: test_function
    // let client = ContractClient::new(&env, &contract_id);
    
    assert!(true);
}

