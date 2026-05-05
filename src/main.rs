use colored::Colorize;
use solana_sdk::{
    message::Message,
    native_token::LAMPORTS_PER_SOL,
    pubkey::Pubkey,
    system_instruction,
    transaction::Transaction,
};
use std::{str::FromStr, time::Duration};
use tokio::time::sleep;

// =====================================================================
// 🔒 MODULE: THE SECURE ENCLAVE (SIMULATED TEE)
// =====================================================================
// In a real environment, this code runs in isolated hardware (Intel SGX/AWS Nitro).
// We use Rust's visibility rules to simulate this. The main thread CANNOT see the key.
mod secure_enclave {
    use solana_sdk::signature::{Keypair, Signer};
    use solana_sdk::transaction::Transaction;

    pub struct SimulatedTEE {
        // Private field! The main program cannot access this.
        locked_keypair: Keypair,
    }

    impl SimulatedTEE {
        // Bootstraps the enclave and generates the key entirely internally
        pub fn initialize() -> Self {
            Self {
                locked_keypair: Keypair::new(),
            }
        }

        // The only publicly exposed data is the public key
        pub fn get_public_key(&self) -> String {
            self.locked_keypair.pubkey().to_string()
        }

        // The Boundary function. The agent passes a raw transaction IN,
        // the enclave signs it internally, and passes the signed transaction OUT.
        pub fn sign_payload(&self, mut tx: Transaction) -> Result<Transaction, String> {
            // Simulated cryptographic delay inside the hardware enclave
            std::thread::sleep(std::time::Duration::from_millis(800));
            
            // Sign the transaction using the isolated keypair
            // We use the pubkey directly from the locked keypair to satisfy the signature check
            let pubkey = self.locked_keypair.pubkey();
            tx.sign(&[&self.locked_keypair], tx.message.recent_blockhash);
            
            Ok(tx)
        }
    }
}

// =====================================================================
// 🤖 MODULE: THE UNTRUSTED AI AGENT (MAIN EXECUTION)
// =====================================================================

#[tokio::main]
async fn main() {
    println!("\n{}", "🛡️  INITIATING TEE-SECURED AI AGENT".bright_blue().bold());
    println!("{}", "==================================================".bright_black());

    // 1. Boot up the hardware enclave
    println!("{} Bootstrapping isolated hardware enclave...", "[SYSTEM]".cyan());
    sleep(Duration::from_millis(500)).await;
    
    let enclave = secure_enclave::SimulatedTEE::initialize();
    let agent_pubkey = enclave.get_public_key();
    
    println!("{} {} Enclave secured. Keypair generated in memory isolation.", "[ENCLAVE]".green().bold(), "✅".green());
    println!("{} Agent Identity (Pubkey): {}\n", "[ENCLAVE]".green().bold(), agent_pubkey.bright_magenta());

    // NOTE: If you try to do `enclave.locked_keypair` here, the Rust compiler will panic.
    // The agent literally cannot steal its own key.

    // 2. The AI Agent decides to execute a trade/payment
    println!("{} Autonomous logic triggered. Goal: Pay external vendor.", "[AGENT]".yellow().bold());
    let target_wallet = Pubkey::new_unique();
    let transfer_amount = (0.25 * LAMPORTS_PER_SOL as f64) as u64;

    println!("{} Constructing raw transaction payload (0.25 SOL)...", "[AGENT]".yellow().bold());
    
    // Agent builds the transaction using the public key it got from the enclave
    let agent_pubkey_struct = Pubkey::from_str(&agent_pubkey).unwrap();
    let instruction = system_instruction::transfer(
        &agent_pubkey_struct,
        &target_wallet,
        transfer_amount,
    );

    // Normally we fetch this from RPC, mocking it for the secure enclave demo
    let mock_blockhash = solana_sdk::hash::Hash::new_unique();
    let message = Message::new(&[instruction], Some(&agent_pubkey_struct));
    
    // The UNSIGNED transaction
    let unsigned_tx = Transaction::new_unsigned(message);

    println!("{} Payload ready. Requesting cryptographic signature from hardware...", "[AGENT]".yellow().bold());
    println!("{}", "--------------------------------------------------".bright_black());

    // 3. Crossing the Boundary into the Enclave
    println!("{} {} Pushing raw bytes across isolation boundary...", "[BOUNDARY]".magenta().bold(), "⬇️".magenta());
    
    match enclave.sign_payload(unsigned_tx) {
        Ok(signed_tx) => {
            println!("{} {} Hardware validation passed. Payload signed internally.", "[ENCLAVE]".green().bold(), "🔒".green());
            println!("{} {} Extracting signed transaction back to main thread...", "[BOUNDARY]".magenta().bold(), "⬆️".magenta());
            println!("{}", "--------------------------------------------------\n".bright_black());
            
            // 4. Agent receives the signed transaction and broadcasts it
            println!("{} Received cryptographically signed transaction.", "[AGENT]".yellow().bold());
            
            // We just grab the first signature to display it
            let sig = signed_tx.signatures.first().unwrap();
            println!("{} Broadcasting to network! Signature: {}", "[AGENT]".yellow().bold(), sig.to_string().bright_blue());
            
            println!("\n{} {} AGENT EXECUTION COMPLETE. KEY NEVER EXPOSED.", "[SUCCESS]".green().bold(), "✅".green());
        },
        Err(e) => {
            println!("{} Enclave rejected signature request: {}", "[FATAL]".red().bold(), e);
        }
    }
}