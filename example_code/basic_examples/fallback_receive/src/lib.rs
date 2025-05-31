#![cfg_attr(not(any(test, feature = "export-abi")), no_main)]
extern crate alloc;

use stylus_sdk::alloy_sol_types::sol;
/// Import items from the SDK. The prelude contains common traits and macros.
use stylus_sdk::{
    alloy_primitives::{U256},
    ArbResult,
    prelude::*,
};
// Define some persistent storage using the Solidity ABI.
// `Counter` will be the entrypoint.
sol_storage! {
    #[entrypoint]
    pub struct Counter {
        uint256 number;
    }
}

// Define events
sol! {
event CounterUpdated(uint256 prev_value, uint256 new_value);
}

/// Declare that `Counter` is a contract with the following external methods.
#[public]
impl Counter {
    pub fn number(&self) -> U256 {
        self.number.get()
    }
    /// Sets a number in storage to a user-specified value.
    pub fn set_number(&mut self, new_number: U256) {
        let prev = self.number.get();
        self.number.set(new_number);
        // Emit an event
        stylus_sdk::stylus_core::log(
            self.vm(),
            CounterUpdated {
                prev_value: prev,
                new_value: self.number.get(),
            },
        );
    }

    #[fallback]
    fn fallback(&mut self, calldata: &[u8]) -> ArbResult {
        let len = calldata.len();
        // Increment the number in storage.
        let prev = self.number.get();
        self.set_number(prev + U256::from(len));
        // Emit an event
        stylus_sdk::stylus_core::log(
            self.vm(),
            CounterUpdated {
                prev_value: prev,
                new_value: self.number.get(),
            },
        );
        Ok(vec![])
    }

    #[receive]
    #[payable]
    fn receive(&mut self) -> Result<(), Vec<u8>> {
        let prev = self.number.get();   
        self.set_number(prev + self.vm().msg_value());     
        // Emit an event
        stylus_sdk::stylus_core::log(
            self.vm(),
            CounterUpdated {
                prev_value: prev,
                new_value: self.number.get(),
            },
        );
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_counter() {
        use stylus_sdk::testing::*;
        let vm = TestVM::default();
        let mut contract = Counter::from(&vm);

        assert_eq!(U256::ZERO, contract.number());

        contract.set_number(U256::from(100));
        assert_eq!(U256::from(100), contract.number());
        
        // Override the msg value for future contract method invocations.
        vm.set_value(U256::from(2));
        let _ =contract.receive();
        assert_eq!(U256::from(102), contract.number());

        let _ =contract.fallback(&[0x01, 0x02, 0x03]);
        assert_eq!(U256::from(105), contract.number());        
    }
}