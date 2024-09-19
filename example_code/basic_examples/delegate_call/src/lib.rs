#![cfg_attr(not(feature = "export-abi"), no_main)]
extern crate alloc;

use alloy_sol_types::sol;
use stylus_sdk::{
    alloy_primitives::Address,
    call::{delegate_call, RawCall},
    prelude::*,
};

#[storage]
#[entrypoint]
pub struct ExampleContract;
// Declare events and Solidity error types
sol! {
    error DelegateCallFailed();
}

#[derive(SolidityError)]
pub enum DelegateCallErrors {
    DelegateCallFailed(DelegateCallFailed),
}

#[public]
impl ExampleContract {
    pub fn low_level_delegate_call(
        &mut self,
        calldata: Vec<u8>,
        target: Address,
    ) -> Result<Vec<u8>, DelegateCallErrors> {
        unsafe {
            let result = delegate_call(self, target, &calldata)
                .map_err(|_| DelegateCallErrors::DelegateCallFailed(DelegateCallFailed {}))?;

            Ok(result)
        }
    }
    pub fn raw_delegate_call(
        &mut self,
        calldata: Vec<u8>,
        target: Address,
    ) -> Result<Vec<u8>, Vec<u8>> {
        let data = RawCall::new_delegate() // configure a delegate call
            .gas(2100) // supply 2100 gas
            .limit_return_data(0, 32) // only read the first 32 bytes back
            .call(target, &calldata)?;

        Ok(data)
    }
}
