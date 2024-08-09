#![no_main]
#![no_std]
extern crate alloc;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

use alloc::vec;
use alloc::vec::Vec;

use stylus_sdk::alloy_primitives::Address;
use stylus_sdk::prelude::*;
use stylus_sdk::storage::StorageAddress;

use stylus_sdk::alloy_primitives::{U16, U256};
use stylus_sdk::prelude::*;
use stylus_sdk::storage::{StorageAddress, StorageBool, StorageU256};
use stylus_sdk::{block, console, msg};


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
    pub fn get_owner(&self) -> StorageAddress {
        self.owner.get()
    }
}

impl ExampleContract {
    // Internal function to set a new owner
    pub fn set_owner(&mut self, new_owner: StorageAddress) {
        self.owner.set(new_owner);
    }

    // Internal function to log data
    pub fn log_data(&self) {
        let data = self.data.get();
        console!("Current data is: {:?}", data);
    }
}