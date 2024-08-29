// Only run this as a WASM if the export-abi feature is not set.
#![cfg_attr(not(any(feature = "export-abi", test)), no_main)]
extern crate alloc;

#[global_allocator]
static ALLOC: mini_alloc::MiniAlloc = mini_alloc::MiniAlloc::INIT;
use alloc::vec::Vec;
use alloc::{string::ToString, vec};

use stylus_sdk::alloy_primitives::U256;
use stylus_sdk::{alloy_primitives::Address, alloy_sol_types::sol, evm, prelude::*, ArbResult};

// sol! macro event declaration
// Up to 3 parameters can be indexed.
// Indexed parameters helps you filter the logs by the indexed parameter
sol! {
    event Log(address indexed sender, string message);
    event AnotherLog();
}

#[solidity_storage]
#[entrypoint]
pub struct Events {
    
}


#[external]
impl Events {
fn user_main(_input: Vec<u8>) -> ArbResult {
    // emits a 'Log' event, defined above in the sol! macro
    evm::log(Log {
        sender: Address::from([0x11; 20]),
        message: "Hello world!".to_string(),
    });

    // no data, but event will still log to the chain
    evm::log(AnotherLog {});

    // set up local variables
    let user = Address::from([0x22; 20]);
    let balance = U256::from(10_000_000);

    // declare up to 4 topics
    // topics must be of type FixedBytes<32>
    let topics = &[user.into_word()];

    // store non-indexed data in a byte Vec
    let mut data: Vec<u8> = vec![];
    // to_be_bytes means 'to big endian bytes'
    data.extend_from_slice(balance.to_be_bytes::<32>().to_vec().as_slice());

    // unwrap() here 'consumes' the Result
    evm::raw_log(topics.as_slice(), data.as_ref()).unwrap();

    Ok(Vec::new())
}
}