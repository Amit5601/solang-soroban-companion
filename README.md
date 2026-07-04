# 🦀 Solang-Soroban Companion

A lightning-fast, native Rust developer companion for inspecting and testing Solang-compiled Soroban smart contracts.

### The Problem
Testing Soroban smart contracts compiled via Hyperledger Solang currently requires tedious, repetitive boilerplate. Developers are forced to manually inspect WASM exports, register contract IDs, and set up mock environments for every single iteration. This friction severely slows down the developer experience (DevEx) and storage operation validation.

### The Solution
`solang-soroban-companion` automates the testing bottleneck. By natively parsing the WebAssembly (`.wasm`) binary payload emitted by Solang, this CLI tool extracts the contract's public API (Export Section) and automatically generates the exact Rust boilerplate required to test it on the Soroban network. 

Zero manual WASM inspection. Zero boilerplate fatigue. 

---

## ⚡ Features

* **Native WASM Parsing:** Validates Magic Bytes and extracts callable contract functions without requiring the full Soroban SDK overhead.
* **Zero-Config Inspection:** Instantly view the public API of any compiled contract.
* **Automated Test Generation:** Compiles a ready-to-run `contract_test.rs` file populated with the correct testing environment setup and function signatures.
* **Idiomatic Safety:** Fully implements `anyhow` for traceable error contexts and graceful handling of corrupted binaries.

---

## 🛠️ Installation

Ensure you have Rust and Cargo installed. Clone this repository and build from source:

```bash
git clone [https://github.com/Amit5601/solang-soroban-companion.git](https://github.com/Amit5601/solang-soroban-companion.git)
cd solang-soroban-companion
cargo build --release

---

### 🚀 Usage
1. Inspect a Compiled Contract

want to see what functions are actually exported inside your compiled .wasm file? Use the inspect command to read the payload.

```bash
cargo run --release -- inspect --path ./examples/dummy.wasm

---

Output:
✅ Success: Loaded file successfully!
📦 File Size: 1513045 bytes
✨ Valid WebAssembly magic number detected.
---
🔍 Scanning contract for callable functions...
  ⚡ test_function
---

2. Generate Test Boilerplate
Stop writing test setups by hand. Point the gen-test command at your compiled contract to automatically generate a contract_test.rs file containing the environment setup and test stubs for every exported function.

```bash
cargo run --release -- gen-test --path ./examples/dummy.wasm

Output:
✅ Success: Created 'contract_test.rs'!

---

🗺️ Roadmap (Upcoming Features)
[ ] State Mocking: Automatically generate mock state transition parameters.

[ ] CI/CD Integration: Add a validate command for automated pipeline checks.

[ ] Crates.io Release: Publish binary for one-line installation (cargo install solang-soroban-companion).

---

⚖️ License
This project is licensed under the Apache License, Version 2.0. See the LICENSE file for details.