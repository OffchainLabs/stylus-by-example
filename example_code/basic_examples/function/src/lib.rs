// Only run this as a WASM if the export-abi feature is not set.
#![cfg_attr(not(any(feature = "export-abi", test)), no_main)]
extern crate alloc;

#[global_allocator]
static ALLOC: mini_alloc::MiniAlloc = mini_alloc::MiniAlloc::INIT;


use alloc::vec;
use stylus_sdk::alloy_primitives::Address;
use stylus_sdk::prelude::*;
use stylus_sdk::storage::StorageAddress;

use stylus_sdk::alloy_primitives::U256;
use stylus_sdk::storage::StorageU256;
use stylus_sdk::console;


#[solidity_storage]
#[entrypoint]
pub struct ExampleContract {
    owner: StorageAddress,
    data: StorageU256,
}

#[external]
impl ExampleContract {
    // External function to set the data
    pub fn set_data(&mut self, value: U256) {
        self.data.set(value);
    }

    // External function to get the data
    pub fn get_data(&self) -> U256 {
        self.data.get()
    }

    // External function to get the contract owner
    pub fn get_owner(&self) -> Address {
        self.owner.get()
    }
}

impl ExampleContract {
    // Internal function to set a new owner
    pub fn set_owner(&mut self, new_owner: Address) {
        self.owner.set(new_owner);
    }

    // Internal function to log data
    pub fn log_data(&self) {
        let _data = self.data.get();
        console!("Current data is: {:?}", _data);
    }
}