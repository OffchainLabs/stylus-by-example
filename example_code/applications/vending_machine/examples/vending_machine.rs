//! Example on how to interact with a deployed `vending machine` program using defaults.
//! This example uses ethers-rs to instantiate the program using a Solidity ABI.
//! Then, it attempts to check the current cupcake balance, give a cupcake via a transaction,
//! and check the balance again. The deployed program is fully written in Rust and compiled to WASM
//! but with Stylus, it is accessible just as a normal Solidity smart contract is via an ABI.

use ethers::{
    middleware::SignerMiddleware,
    prelude::abigen,
    providers::{Http, Middleware, Provider},
    signers::{LocalWallet, Signer},
    types::Address,
};
use eyre::eyre;
use std::io::{BufRead, BufReader};
use std::str::FromStr;
use std::sync::Arc;
use dotenv::dotenv;
use std::env;

/// Your private key file path.
const PRIV_KEY_PATH: &str = "PRIV_KEY_PATH";

/// Stylus RPC endpoint url.
const RPC_URL: &str = "RPC_URL";

/// Deployed pragram address.
const STYLUS_PROGRAM_ADDRESS: &str = "STYLUS_PROGRAM_ADDRESS";
const USER_ADDRESS: &str = "USER_ADDRESS";

#[tokio::main]
async fn main() -> eyre::Result<()> {
    // Load environment variables from .env file
    dotenv().ok();

     // Debugging: Print environment variables
     println!("PRIV_KEY_PATH: {:?}", env::var(PRIV_KEY_PATH));
     println!("RPC_URL: {:?}", env::var(RPC_URL));
     println!("STYLUS_PROGRAM_ADDRESS: {:?}", env::var(STYLUS_PROGRAM_ADDRESS));
     println!("USER_ADDRESS: {:?}", env::var(USER_ADDRESS));
    
    let priv_key_path = env::var(PRIV_KEY_PATH).map_err(|_| eyre!("No {} env var set", PRIV_KEY_PATH))?;
    let rpc_url = env::var(RPC_URL).map_err(|_| eyre!("No {} env var set", RPC_URL))?;
    let program_address = env::var(STYLUS_PROGRAM_ADDRESS)
        .map_err(|_| eyre!("No {} env var set", STYLUS_PROGRAM_ADDRESS))?;
    let user_address_str = env::var(USER_ADDRESS).map_err(|_| eyre!("No {} env var set", USER_ADDRESS))?;
    let user_address: Address = user_address_str.parse().map_err(|e| eyre!("Failed to parse user address: {}", e))?;




    abigen!( //abigen! macro is used to generate type-safe bindings to the Counter smart contract based on its ABI
        VendingMachine,
        r#"[
            function giveCupcakeTo(address user_address) external returns (bool)
            function getCupcakeBalanceFor(address user_address) external view returns (uint256)
        ]"#
    );

    let provider = Provider::<Http>::try_from(rpc_url)?;
    let address: Address = program_address.parse()?;

    let privkey = read_secret_from_file(&priv_key_path)?;
    println!("Private key read from file: {}", privkey); // Debugging line


    let wallet = LocalWallet::from_str(&privkey)?;
    let chain_id = provider.get_chainid().await?.as_u64();
    let client = Arc::new(SignerMiddleware::new(
        provider,
        wallet.clone().with_chain_id(chain_id),
    ));

    let vending_machine = VendingMachine::new(address, client);

    let balance = vending_machine.get_cupcake_balance_for(user_address).call().await?;
    println!("User cupcake balance = {:?}", balance);


    let tx_receipt = vending_machine.give_cupcake_to(user_address).send().await?.await?;
    match tx_receipt {
        Some(receipt) => {
            if receipt.status == Some(1.into()) {
                println!("Successfully gave cupcake to user via a tx");
            } else {
                println!("Failed to give cupcake to user, tx failed");
            }
        }
        None => {
            println!("Failed to get transaction receipt");
        }
    }

    let balance = vending_machine.get_cupcake_balance_for(user_address).call().await?;
    println!("New user cupcake balance = {:?}", balance);

    Ok(())
}

fn read_secret_from_file(fpath: &str) -> eyre::Result<String> {
    let f = std::fs::File::open(fpath)?;
    let mut buf_reader = BufReader::new(f);
    let mut secret = String::new();
    buf_reader.read_line(&mut secret)?;
    Ok(secret.trim().to_string())
}