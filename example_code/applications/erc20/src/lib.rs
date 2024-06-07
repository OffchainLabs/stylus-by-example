// Only run this as a WASM if the export-abi feature is not set.
#![cfg_attr(not(any(feature = "export-abi", test)), no_main)]
extern crate alloc;

// Modules and imports
mod erc20;

use alloy_primitives::{Address, U256};
use stylus_sdk::{
    msg,
    prelude::*
};
use crate::erc20::{Erc20, Erc20Params, Erc20Error};

/// Initializes a custom, global allocator for Rust programs compiled to WASM.
#[global_allocator]
static ALLOC: mini_alloc::MiniAlloc = mini_alloc::MiniAlloc::INIT;

/// Immutable definitions
struct StylusTokenParams;
impl Erc20Params for StylusTokenParams {
    const NAME: &'static str = "StylusToken";
    const SYMBOL: &'static str = "STK";
    const DECIMALS: u8 = 18;
}

// Define the entrypoint as a Solidity storage object. The sol_storage! macro
// will generate Rust-equivalent structs with all fields mapped to Solidity-equivalent
// storage slots and types.
sol_storage! {
    #[entrypoint]
    struct StylusToken {
        // Allows erc20 to access StylusToken's storage and make calls
        #[borrow]
        Erc20<StylusTokenParams> erc20;
    }
}

#[external]
#[inherit(Erc20<StylusTokenParams>)]
impl StylusToken {
    /// Mints tokens
    pub fn mint(&mut self, to: Address, value: U256) -> Result<(), Erc20Error> {
        self.erc20.mint(to, value)?;
        Ok(())
    }

    /// Burns tokens
    pub fn burn(&mut self, value: U256) -> Result<(), Erc20Error> {
        self.erc20.burn(msg::sender(), value)?;
        Ok(())
    }
}
