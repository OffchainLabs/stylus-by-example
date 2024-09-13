#![cfg_attr(not(feature = "export-abi"), no_main)]
extern crate alloc;

use stylus_sdk::{
    alloy_primitives::Address,
    call::{call, static_call, Call, RawCall},
    evm, msg,
    prelude::*,
};

sol_interface! {
    interface IService {
        function makePayment(address user) payable external returns (string);
        function getConstant() pure external returns (bytes32);
    }

    interface IMethods {
        function pureFoo() external pure;
        function viewFoo() external view;
        function writeFoo() external;
        function payableFoo() external payable;
    }
}

#[storage]
#[entrypoint]
pub struct ExampleContract;
#[public]
impl ExampleContract {
    // simple call to contract using interface
    pub fn simple_call(&mut self, account: IService, user: Address) -> Result<String, Vec<u8>> {
        // Calls the make_payment method
        Ok(account.make_payment(self, user)?)
    }
    #[payable]
    // configuring gas and value with Call
    pub fn call_with_gas_value(
        &mut self,
        account: IService,
        user: Address,
    ) -> Result<String, Vec<u8>> {
        let config = Call::new_in(self)
            .gas(evm::gas_left() / 2) // Use half the remaining gas
            .value(msg::value()); // Use the transferred value

        Ok(account.make_payment(config, user)?)
    }

    pub fn call_pure(&self, methods: IMethods) -> Result<(), Vec<u8>> {
        Ok(methods.pure_foo(self)?) // `pure` methods might lie about not being `view`
    }

    pub fn call_view(&self, methods: IMethods) -> Result<(), Vec<u8>> {
        Ok(methods.view_foo(self)?)
    }

    pub fn call_write(&mut self, methods: IMethods) -> Result<(), Vec<u8>> {
        methods.view_foo(&mut *self)?; // Re-borrow `self` to avoid moving it
        Ok(methods.write_foo(self)?) // Safely use `self` again for write_foo
    }

    #[payable]
    pub fn call_payable(&mut self, methods: IMethods) -> Result<(), Vec<u8>> {
        methods.write_foo(Call::new_in(self))?; // these are the same
        Ok(methods.payable_foo(self)?) // ------------------
    }

    // When writing Stylus libraries, a type might not be TopLevelStorage and therefore &self or &mut self won’t work. Building a Call from a generic parameter via new_in is the usual solution.
    pub fn make_generic_call(
        storage: &mut impl TopLevelStorage, // This could be `&mut self`, or another type implementing `TopLevelStorage`
        account: IService,                  // Interface for calling the target contract
        user: Address,
    ) -> Result<String, Vec<u8>> {
        let config = Call::new_in(storage) // Take exclusive access to all contract storage
            .gas(evm::gas_left() / 2) // Use half the remaining gas
            .value(msg::value()); // Use the transferred value

        Ok(account.make_payment(config, user)?) // Call using the configured parameters
    }

    // Low level Call
    pub fn execute_call(
        &mut self,
        contract: Address,
        calldata: Vec<u8>, // Calldata is supplied as a Vec<u8>
    ) -> Result<Vec<u8>, Vec<u8>> {
        // Perform a low-level `call`
        let return_data = call(
            Call::new_in(self) // Configuration for gas, value, etc.
                .gas(evm::gas_left() / 2), // Use half the remaining gas
            contract,  // The target contract address
            &calldata, // Raw calldata to be sent
        )?;

        // Return the raw return data from the contract call
        Ok(return_data)
    }

    // Low level Static Call
    pub fn execute_static_call(
        &mut self,
        contract: Address,
        calldata: Vec<u8>,
    ) -> Result<Vec<u8>, Vec<u8>> {
        // Perform a low-level `static_call`, which does not modify state
        let return_data = static_call(
            Call::new_in(self), // Configuration for the call
            contract,           // Target contract
            &calldata,          // Raw calldata
        )?;

        // Return the raw result data
        Ok(return_data)
    }

    // Using Unsafe RawCall
    pub fn raw_call_example(
        &mut self,
        contract: Address,
        calldata: Vec<u8>,
    ) -> Result<Vec<u8>, Vec<u8>> {
        unsafe {
            let data = RawCall::new_delegate()
                .gas(2100) // Set gas to 2100
                .limit_return_data(0, 32) // Limit return data to 32 bytes
                .flush_storage_cache() // flush the storage cache before the call
                .call(contract, &calldata)?; // Execute the call
            Ok(data) // Return the raw result
        }
    }
}
