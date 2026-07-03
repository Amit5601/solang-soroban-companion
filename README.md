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

---

## 🛠️ Installation

Ensure you have Rust and Cargo installed. Clone this repository and build from source:

```bash
git clone [https://github.com/Amit5601/solang-soroban-companion.git](https://github.com/Amit5601/solang-soroban-companion.git)
cd solang-soroban-companion
cargo build --release