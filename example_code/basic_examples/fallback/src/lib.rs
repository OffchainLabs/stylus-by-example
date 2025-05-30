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
        let _ = self.set_implementation(new_implementation);
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

    /// Delegate call to implementation
    pub fn delegate(&mut self, calldata: Vec<u8>) -> Result<Vec<u8>, ProxyErrors> {
        let implementation = self.logic_contract.get();
        
        if implementation.is_zero() {
            return Err(ProxyErrors::InvalidImplementation(InvalidImplementation {}));
        }
        
        // Perform delegatecall
        unsafe {
            let result = self.vm()
            .delegate_call(&self, implementation, &calldata)
            .map_err(|_| ProxyErrors::DelegateCallFailed(DelegateCallFailed {}))?;
            Ok(result)
        }
    }

    /// Internal function to set implementation
    fn set_implementation(&mut self, new_implementation: Address) -> Result<(), ProxyErrors> {
        if new_implementation.is_zero() {
            return Err(ProxyErrors::InvalidImplementation(InvalidImplementation {}));
        }

        self.logic_contract.set(new_implementation);
        
        stylus_sdk::stylus_core::log(
            self.vm(),
            Upgraded {
            implementation: new_implementation,
        });

        Ok(())
    }

    #[fallback]
    fn fallback(&mut self, calldata: &[u8]) -> ArbResult {
        let result= self.delegate(calldata.to_vec())?;
        Ok(result)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use alloy_primitives::address;

    #[test]
    fn test_proxy_initialization() {
        let logic = address!("0x1234567890123456789012345678901234567890");
        let admin = address!("0xabcdefabcdefabcdefabcdefabcdefabcdefabcd");
        let mut proxy = Proxy::default(); //constructor

        assert_eq!(proxy.implementation().unwrap(), logic);
        assert_eq!(proxy.admin().unwrap(), admin);
    }
}