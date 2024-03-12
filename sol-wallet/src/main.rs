use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    pubkey::Pubkey,
    signature::{Keypair, Signer},
    transaction::Transaction,
};

fn main() {
    // Generate a new wallet address
    let wallet_keypair = Keypair::new();
    println!("New wallet address: {}", wallet_keypair.pubkey());

    // Display wallet balance
    let client = RpcClient::new("https://api.devnet.solana.com".to_string());
    let balance = client.get_balance(&wallet_keypair.pubkey()).unwrap();
    println!("Wallet balance: {}", lamports_to_sol(balance));

    // Send SOL to another address
    // Example: Sending 0.1 SOL to a recipient
    // Ensure you replace `recipient_pubkey_str` with the actual recipient's public key
    let recipient_pubkey_str = "RecipientPublicKeyHere";
    let _recipient_pubkey = recipient_pubkey_str.parse::<Pubkey>().unwrap();
    let _amount = sol_to_lamports(0.1); // Converts SOL to lamports, as transactions use lamports

    // Create and sign the transaction (simplified for clarity)
    let transaction = Transaction::new_signed_with_payer(
        &[/* Add instruction to transfer SOL here */],
        Some(&wallet_keypair.pubkey()),
        &[&wallet_keypair],
        client.get_latest_blockhash().unwrap(),
    );

    // Send the transaction
    let signature = client.send_and_confirm_transaction(&transaction).unwrap();
    println!("Transaction signature: {}", signature);
}

// Helper functions to convert between lamports and SOL
fn lamports_to_sol(lamports: u64) -> f64 {
    lamports as f64 / 1_000_000_000.0
}

fn sol_to_lamports(sol: f64) -> u64 {
    (sol * 1_000_000_000.0) as u64
}
