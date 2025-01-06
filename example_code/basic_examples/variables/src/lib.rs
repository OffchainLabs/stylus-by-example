// Only run this as a WASM if the export-abi feature is not set.
#![cfg_attr(not(any(feature = "export-abi", test)), no_main)]
extern crate alloc;

use stylus_sdk::alloy_primitives::{U16, U256};
use stylus_sdk::prelude::*;
use stylus_sdk::storage::{StorageAddress, StorageBool, StorageU256, StorageString};
use stylus_sdk::{block, console, msg};

#[storage]
#[entrypoint]
pub struct Contract {
    initialized: StorageBool,
    owner: StorageAddress,
    max_supply: StorageU256,
    base_uri: StorageString,
}

#[public]
impl Contract {
    // State variables are initialized in an `init` function.
    pub fn init(&mut self) -> Result<(), Vec<u8>> {
        // We check if contract has been initialized before.
        // We return if so, we initialize if not.
        let initialized = self.initialized.get();
        if initialized {
            return Ok(());
        }
        self.initialized.set(true);

        // We set the contract owner to the caller,
        // which we get from the global msg module
        self.owner.set(msg::sender());
        self.max_supply.set(U256::from(10_000));

        // We set the base URI for the contract
        // Unlike other variables, string needs to use .set_str() to set the value
        self.base_uri.set_str("https://stylus-by-example.org/".to_string());

        Ok(())
    }

    pub fn do_something() -> Result<(), Vec<u8>> {
        // Local variables are not saved to the blockchain
        // 16-bit Rust integer
        let _i = 456_u16;
        // 16-bit int inferred from U16 Alloy primitive
        let _j = U16::from(123);

        // Here are some global variables
        let _timestamp = block::timestamp();
        let _amount = msg::value();

        console!("Local variables: {_i}, {_j}");
        console!("Global variables: {_timestamp}, {_amount}");

        Ok(())
    }

    pub fn get_base_uri(&self) -> Result<String, Vec<u8>> {
        // Unlike other variables, string needs to use .get_string() to get the value
        Ok(self.base_uri.get_string())
    }
}
