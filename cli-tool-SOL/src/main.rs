use anyhow::Result;
use clap::{App, Arg, SubCommand};
use solana_client::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;

fn main() -> Result<()> {
    let matches = App::new("Solana CLI Tool")
        .version("0.1.0")
        .author("Your Name")
        .about("Interacts with Solana blockchain data")
        .subcommand(
            SubCommand::with_name("balance")
                .about("Gets the balance of a Solana address")
                .arg(
                    Arg::with_name("ADDRESS")
                        .help("The Solana address to query")
                        .required(true)
                        .index(1),
                ),
        )
        .subcommand(
            SubCommand::with_name("recent-transactions").about("Fetches recent transactions"),
        )
        .get_matches();

    let client = RpcClient::new("https://api.devnet.solana.com".to_string());

    match matches.subcommand() {
        ("balance", Some(sub_m)) => {
            let address = sub_m.value_of("ADDRESS").unwrap();
            get_balance(&client, address)?;
        }
        ("recent-transactions", Some(_)) => {
            let address = "7MtZxdDX3zhQtNvFeDswqZWordrTMLsHXgwEtBBt47Kc"; // Add a valid Solana address here
            get_recent_transactions(&client, address)?;
        }
        _ => eprintln!("Invalid command"),
    }

    Ok(())
}

fn get_balance(client: &RpcClient, address: &str) -> Result<()> {
    let pubkey = address.parse().expect("Invalid address");
    let balance = client.get_balance(&pubkey)?;
    println!("Balance for {}: {} SOL", address, balance);
    Ok(())
}

fn get_recent_transactions(client: &RpcClient, address: &str) -> Result<()> {
    let address_pubkey = address.parse::<Pubkey>()?; // Parse the address string into a Pubkey
    let transactions = client.get_signatures_for_address(&address_pubkey)?; // Use the parsed Pubkey here

    for transaction in transactions.iter() {
        println!("Transaction Signature: {}", transaction.signature);
    }

    Ok(())
}
