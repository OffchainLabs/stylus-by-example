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

const OWNER: &str = "0xd8dA6BF26964aF9D7eEd9e03E53415D37aA96045";

#[solidity_storage]
#[entrypoint]
pub struct Contract {
    owner: StorageAddress,
}

#[external]
impl Contract {
    pub fn init(&mut self) -> Result<(), Vec<u8>> {
        // Parse the const &str as a local Address variable
        let owner_address = Address::parse_checksummed(OWNER, None).expect("Invalid address");

        // Save the result as the owner
        self.owner.set(owner_address);

        Ok(())
    }
    pub fn owner(&self) -> Result<Address, Vec<u8>> {
        let owner_address = self.owner.get();

        Ok(owner_address)
    }
}
