#![cfg_attr(not(feature = "export-abi"), no_std)]
extern crate alloc;

use alloc::{vec, vec::Vec};
use stylus_sdk::{
    alloy_primitives::Address,
    alloy_sol_types::sol,
    ArbResult,
    prelude::*,
};

// Define storage layout
sol_storage! {
    #[entrypoint]
    pub struct Proxy {
        address logic_contract;
        address admin;
    }
}

// Events
sol! {
    event AdminChanged(address indexed previousAdmin, address indexed newAdmin);
    event Upgraded(address indexed implementation);
    error Unauthorized();
    error InvalidImplementation();
    error DelegateCallFailed();
}


#[derive(SolidityError)]
pub enum ProxyErrors {
    Unauthorized(Unauthorized),
    InvalidImplementation(InvalidImplementation),   
    DelegateCallFailed(DelegateCallFailed),
}

#[public]
impl Proxy {
    /// Constructor to initialize the proxy
    /// Initialize the proxy with a logic contract and admin address
    #[constructor]
    pub fn constructor(&mut self, implementation_address: Address, admin_address: Address){
        // Initialize storage slots
        self.logic_contract.set(implementation_address);
        self.admin.set(admin_address);
    }

    /// Get the current implementation address
    pub fn implementation(&self) -> Result<Address, Vec<u8>> {
        Ok(self.logic_contract.get())
    }

    /// Get the current admin address
    pub fn admin(&self) -> Result<Address, Vec<u8>> {
        Ok(self.admin.get())
    }

    /// Upgrade the implementation (admin only)
    pub fn upgrade_to(&mut self, new_implementation: Address) -> Result<(),ProxyErrors> {
        if self.vm().msg_sender() != self.admin.get() {
            return Err(ProxyErrors::Unauthorized(Unauthorized {}));

        }
        // Check if the new implementation address is valid
        if new_implementation.is_zero() {
            return Err(ProxyErrors::InvalidImplementation(InvalidImplementation {}));
        }
        // Set the new implementation address
        self.logic_contract.set(new_implementation);
        
        stylus_sdk::stylus_core::log(
            self.vm(),
            Upgraded {
            implementation: new_implementation,
        });

        Ok(())
    }
    
    /// Change the admin address (admin only)
    pub fn change_admin(&mut self, new_admin: Address) -> Result<(), ProxyErrors> {
        if self.vm().msg_sender() != self.admin.get() {
            return Err(ProxyErrors::Unauthorized(Unauthorized {}));
        }  

        let previous_admin = self.admin.get();
        self.admin.set(new_admin);
        
        stylus_sdk::stylus_core::log(
            self.vm(),
            AdminChanged {
            previousAdmin: previous_admin,
            newAdmin: new_admin,
        });

        Ok(())
    }

    #[fallback]
    fn fallback(&mut self, calldata: &[u8]) -> ArbResult {
        let implementation = self.logic_contract.get();
        
        // Perform delegatecall
        unsafe {
            let result = self.vm()
            .delegate_call(&self, implementation, &calldata)
            .map_err(|_| ProxyErrors::DelegateCallFailed(DelegateCallFailed {}))?;
            Ok(result)
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use alloy_primitives::address;

    #[test]
    fn test_proxy_initialization() {
        use stylus_sdk::testing::*;
        let vm = TestVM::new();
        let logic = address!("0x1234567890123456789012345678901234567890");
        let admin = vm.msg_sender();
        let mut contract = Proxy::from(&vm);
        contract.constructor(logic, admin);
        assert_eq!(contract.implementation().unwrap(), logic);
        assert_eq!(contract.admin().unwrap(), admin);
    }
    
    #[test]
    fn test_proxy_fallback() {
        use stylus_sdk::testing::*;
        let vm = TestVM::new();
        let mut contract = Proxy::from(&vm);
        let logic = address!("0x1234567890123456789012345678901234567890");
        let admin = vm.msg_sender();
        contract.constructor(logic, admin);
        let success_ret = vec![5, 6, 7, 8];

        // Fallback call
        let data = vec![0x01, 0x02, 0x03]; // Example calldata

        vm.mock_delegate_call(logic, data.clone(), Ok(success_ret.clone()));
        let result = contract.fallback(&data.clone());
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), success_ret);
    }

    #[test]
    fn test_proxy_upgrade() {
        use stylus_sdk::testing::*;
        let vm = TestVM::new();
        let mut contract = Proxy::from(&vm);
        let logic = address!("0x1234567890123456789012345678901234567890");
        let new_logic = address!("0x0987654321098765432109876543210987654321");
        let admin = vm.msg_sender();
        contract.constructor(logic, admin);
        
        // Upgrade implementation
        let _ = contract.upgrade_to(new_logic);
        assert_eq!(contract.implementation().unwrap(), new_logic);
    }
    #[test]
    fn test_proxy_change_admin() {
        use stylus_sdk::testing::*;
        let vm = TestVM::new();
        let mut contract = Proxy::from(&vm);
        let logic = address!("0x1234567890123456789012345678901234567890");
        let admin = vm.msg_sender();
        contract.constructor(logic, admin);
        
        // Change admin
        let new_admin = address!("0x0987654321098765432109876543210987654321");
        let _ = contract.change_admin(new_admin);
        assert_eq!(contract.admin().unwrap(), new_admin);
    }
}