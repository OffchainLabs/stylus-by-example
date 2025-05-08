#![cfg_attr(not(any(test, feature = "export-abi")), no_main)]
extern crate alloc;
/// Import items from the SDK. The prelude contains common traits and macros.
use stylus_sdk::{alloy_primitives::U256, prelude::*};
// Define some persistent storage using the Solidity ABI.
// `Counter` will be the entrypoint.
sol_storage! {
    #[entrypoint]
    pub struct Counter {
        uint256 number;
    }
}
/// Declare that `Counter` is a contract with the following external methods.
#[public]
impl Counter {
    /// Initializes the contract with a constructor parameter.
    #[constructor]
    pub fn constructor(&mut self, new_number: U256) {
        self.number.set(new_number)
    }

    /// Gets the number from storage.
    pub fn number(&self) -> U256 {
        self.number.get()
    }
    /// Sets a number in storage to a user-specified value.
    pub fn set_number(&mut self, new_number: U256) {
        self.number.set(new_number);
    
    }
}