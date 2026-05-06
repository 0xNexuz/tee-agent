# 🛡️ TEE-Secured AI Agent

A Rust-based autonomous execution engine utilizing a simulated Trusted Execution Environment (TEE) for bank-grade key management.

As AI agents begin holding substantial Total Value Locked (TVL), storing private keys in plaintext environment variables is a critical vulnerability. This project demonstrates a secure enclave architecture using Rust's strict memory isolation and visibility rules. The AI agent can request the hardware enclave to sign a transaction, but the main execution thread is cryptographically blocked from ever accessing the raw private key.

## ✨ Features

- **Memory Isolation:** The Solana keypair is generated and stored entirely within a restricted module. The compiler physically prevents the main agent thread from extracting the key.
- **Cryptographic Boundaries:** Simulates an AWS Nitro or Intel SGX hardware boundary. Raw payloads go in, cryptographically signed transactions come out.
- **Autonomous Execution:** The untrusted agent dynamically constructs Solana transactions and negotiates with the secure enclave for signature approval.
- **Zero-Exposure Architecture:** Even if the server environment is fully compromised, the wallet's private key cannot be scraped from the agent's memory state.

## 🚀 Quick Start

### Prerequisites
* Rust & Cargo installed.

### Installation
1. Clone the repository:
   ```bash
   git clone [https://github.com/YOUR_USERNAME/tee-agent.git](https://github.com/YOUR_USERNAME/tee-agent.git)
   cd tee-agent
