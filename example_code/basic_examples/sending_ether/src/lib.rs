// Only run this as a WASM if the export-abi feature is not set.
#![cfg_attr(not(any(feature = "export-abi", test)), no_main)]
extern crate alloc;

use alloy_primitives::Address;
use stylus_sdk::{
    abi::Bytes,
    call::{call, transfer_eth, Call},
    msg::{self},
    prelude::*,
};

sol_interface! {
    interface ITarget {
        function receiveEther() external payable;
    }
}

#[storage]
#[entrypoint]
pub struct SendEther {}

#[public]
impl SendEther {
    // Transfer Ether using the transfer_eth method
    // This can be used to send Ether to an EOA or a Solidity smart contract that has a receive() function implemented
    #[payable]
    pub fn send_via_transfer(to: Address) -> Result<(), Vec<u8>> {
        transfer_eth(to, msg::value())?;
        Ok(())
    }

    // Transfer Ether using a low-level call
    // This can be used to send Ether to an EOA or a Solidity smart contract that has a receive() function implemented
    #[payable]
    pub fn send_via_call(&mut self, to: Address) -> Result<(), Vec<u8>> {
        call(Call::new_in(self).value(msg::value()), to, &[])?;
        Ok(())
    }

    // Transfer Ether using a low-level call with a specified gas limit
    // This can be used to send Ether to an EOA or a Solidity smart contract that has a receive() function implemented
    #[payable]
    pub fn send_via_call_gas_limit(&mut self, to: Address, gas_amount: u64) -> Result<(), Vec<u8>> {
        call(
            Call::new_in(self).value(msg::value()).gas(gas_amount),
            to,
            &[],
        )?;
        Ok(())
    }

    // Transfer Ether using a low-level call with calldata
    // This can be used to call a Solidity smart contract's fallback function and send Ether along with calldata
    #[payable]
    pub fn send_via_call_with_call_data(
        &mut self,
        to: Address,
        data: Bytes,
    ) -> Result<(), Vec<u8>> {
        call(Call::new_in(self).value(msg::value()), to, data.as_slice())?;
        Ok(())
    }

    // Transfer Ether to another smart contract via a payable method on the target contract
    // The target contract can be either a Solidity smart contract or a Stylus contract that has a receiveEther function, which is a payable function
    #[payable]
    pub fn send_to_stylus_contract(&mut self, to: Address) -> Result<(), Vec<u8>> {
        let target = ITarget::new(to);
        let config = Call::new_in(self).value(msg::value());
        target.receive_ether(config)?;
        Ok(())
    }
}
