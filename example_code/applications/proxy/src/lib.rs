#![cfg_attr(not(feature = "export-abi"), no_std)]
extern crate alloc;

use alloc::{vec, vec::Vec};
use stylus_sdk::{
    stylus_core::log,
    alloy_primitives::{Address, B256, U256, b256},
    alloy_sol_types::sol,
    ArbResult,
    prelude::*,
};

/* ---------- 1. module-level constants ---------- */

/// EIP-1967 implementation slot = keccak256("eip1967.proxy.implementation") – 1
const IMPLEMENTATION_SLOT: B256 =
    b256!("360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc");

/// EIP-1967 admin slot = keccak256("eip1967.proxy.admin") – 1
const ADMIN_SLOT: B256 =
    b256!("b53127684a568b3173ae13b9f8a6016e243e63b6e8ee1178d6a717850b5d6103");

// Define storage layout
sol_storage! {
    #[entrypoint]
    pub struct Proxy {
    }
}

// Events
sol! {
    event AdminChanged(address indexed previousAdmin, address indexed newAdmin);
    event Upgraded(address indexed implementation);
    error Unauthorized();
    error InvalidImplementation();
    error InvalidAdmin();
    error DelegateCallFailed();
    error ValueSentNotAllowed();
}


#[derive(SolidityError)]
pub enum ProxyErrors {
    Unauthorized(Unauthorized),
    InvalidImplementation(InvalidImplementation),
    InvalidAdmin(InvalidAdmin),   
    DelegateCallFailed(DelegateCallFailed),
    ValueSentNotAllowed(ValueSentNotAllowed),
}

#[public]
impl Proxy {

    /// Constructor to initialize the proxy
    /// Initialize the proxy with a logic contract and admin address
    #[constructor]
    pub fn constructor(&mut self, implementation_address: Address, admin_address: Address) -> Result<(),ProxyErrors> {
        // Check if the implementation address is valid
        if implementation_address.is_zero() {
            return Err(ProxyErrors::InvalidImplementation(InvalidImplementation {}));
        }
        // Check if the admin address is valid
        if admin_address.is_zero() {
            return Err(ProxyErrors::InvalidAdmin(InvalidAdmin {}));
        }
        // Set the implementation contract address to EIP-1967 implementation slot
        let new_impl = U256::from_be_bytes(implementation_address.into_word().into());
        unsafe {
            self.vm().storage_cache_bytes32(IMPLEMENTATION_SLOT.into(), B256::from_slice(&new_impl.to_be_bytes::<32>()));
        }
        self.vm().flush_cache(false);
        // Emit the Upgraded event
        log(
            self.vm(),
            Upgraded {
                implementation: implementation_address,
            });
        // Set the admin address to EIP-1967 admin slot
        let new_admin = U256::from_be_bytes(admin_address.into_word().into());
        unsafe {
            self.vm().storage_cache_bytes32(ADMIN_SLOT.into(), B256::from_slice(&new_admin.to_be_bytes::<32>()));
        }
        self.vm().flush_cache(false);
        // Emit the AdminChanged event
        log(
            self.vm(),
            AdminChanged {
                previousAdmin: Address::ZERO,
                newAdmin: admin_address,
            });
        // Return success
        Ok(())
    }

    /// Get the current implementation address
    pub fn implementation(&self) -> Result<Address, Vec<u8>> {
        let raw = self.vm().storage_load_bytes32(IMPLEMENTATION_SLOT.into());
        let extracted_address = Address::from_slice(&raw[12..]);
        // Return the implementation address
        Ok(extracted_address)
    }

    /// Get the current admin address
    pub fn admin(&self) -> Result<Address, Vec<u8>> {
        let raw = self.vm().storage_load_bytes32(ADMIN_SLOT.into());
        let extracted_address = Address::from_slice(&raw[12..]);
        // Return the implementation address
        Ok(extracted_address)
    }

    /// Upgrade the implementation (admin only)
    pub fn upgrade_to(&mut self, new_implementation: Address) -> Result<(),ProxyErrors> {
        // Get the current admin address
        let admin_address = self.admin().map_err(|_| ProxyErrors::Unauthorized(Unauthorized {}))?;
        // Check if the caller is the admin        
        if self.vm().msg_sender() != admin_address {
            return Err(ProxyErrors::Unauthorized(Unauthorized {}));

        }
        // Check if the new implementation address is valid
        if new_implementation.is_zero() {
            return Err(ProxyErrors::InvalidImplementation(InvalidImplementation {}));
        }
        // Set the new implementation address
        let new_impl = U256::from_be_bytes(new_implementation.into_word().into());
        unsafe {
            self.vm().storage_cache_bytes32(IMPLEMENTATION_SLOT.into(), B256::from_slice(&new_impl.to_be_bytes::<32>()));
        }
        self.vm().flush_cache(false);
        // Emit the Upgraded event        
        log(
            self.vm(),
            Upgraded {
            implementation: new_implementation,
        });

        Ok(())
    }
    
    pub fn change_admin(&mut self, new_admin: Address) -> Result<(), ProxyErrors> {
        // Get the current admin address
        let admin_address = self.admin().map_err(|_| ProxyErrors::Unauthorized(Unauthorized {}))?;
        // Check if the caller is the admin
        if self.vm().msg_sender() != admin_address {
            return Err(ProxyErrors::Unauthorized(Unauthorized {}));
        }  
        // Check if the new admin address is valid
        if new_admin.is_zero() {
            return Err(ProxyErrors::InvalidAdmin(InvalidAdmin {}));
        }
        // Set the new admin address
        let new_admin = U256::from_be_bytes(new_admin.into_word().into());
        unsafe {
            self.vm().storage_cache_bytes32(ADMIN_SLOT.into(), B256::from_slice(&new_admin.to_be_bytes::<32>()));
        }
        self.vm().flush_cache(false);
        // Get the previous admin address
        let new_admin = Address::from_slice(&new_admin.to_be_bytes::<32>()[12..]);
        // Emit the AdminChanged event
        log(
            self.vm(),
            AdminChanged {
            previousAdmin: admin_address,
            newAdmin: new_admin,
        });

        Ok(())
    }

    #[fallback]
    #[payable]
    fn fallback(&mut self, calldata: &[u8]) -> ArbResult {
        // Get the implementation address
        let implementation = self.implementation()?;        
        // Perform delegatecall
        unsafe {
            let result = self.vm()
            .delegate_call(&self, implementation, &calldata)
            .map_err(|_| ProxyErrors::DelegateCallFailed(DelegateCallFailed {}))?;
            Ok(result)
        }
    }

    #[receive]
    #[payable]
    fn receive(&mut self) -> Result<(), Vec<u8>> {
        if self.vm().msg_value() > U256::ZERO {
            return Err(ProxyErrors::ValueSentNotAllowed(ValueSentNotAllowed {}).into());
        }
        Ok(())      
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
        let mut contract = Proxy::from(&vm);
        let logic = address!("0x1234567890123456789012345678901234567890");
        let admin = vm.msg_sender();
        let _ = contract.constructor(logic, admin);
        // Check if the implementation address is set correctly
        assert_eq!(contract.implementation().unwrap(), logic);
        // Check if the admin address is set correctly
        assert_eq!(contract.admin().unwrap(), admin);
    }
    
    #[test]
    fn test_proxy_fallback() {
        use stylus_sdk::testing::*;
        let vm = TestVM::new();
        let mut contract = Proxy::from(&vm);
        let logic = address!("0x1234567890123456789012345678901234567890");
        let admin = vm.msg_sender();
        let _ = contract.constructor(logic, admin);
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
        let _ = contract.constructor(logic, admin);
        
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
        let _ = contract.constructor(logic, admin);
        
        // Change admin
        let new_admin = address!("0x0987654321098765432109876543210987654321");
        let _ = contract.change_admin(new_admin);
        assert_eq!(contract.admin().unwrap(), new_admin);
    }
    #[test]
    fn test_proxy_unauthorized_upgrade() {
        use stylus_sdk::testing::*;
        let vm = TestVM::new();
        let mut contract = Proxy::from(&vm);
        let logic = address!("0x1234567890123456789012345678901234567890");
        let admin = address!("0x0987654321098765432109876543210987654321"); 
        let _ = contract.constructor(logic, admin);
        // Attempt to upgrade implementation as a non-admin msg sender
        let result = contract.upgrade_to(address!("0x0987654321098765432109876543210987654321"));
        assert!(result.is_err());
        assert!(matches!(result, Err(ProxyErrors::Unauthorized(_))));
    }
    
    #[test]
    fn test_proxy_unauthorized_change_admin() {
        use stylus_sdk::testing::*;
        let vm = TestVM::new();
        let mut contract = Proxy::from(&vm);
        let logic = address!("0x1234567890123456789012345678901234567890");
        let admin = address!("0x0987654321098765432109876543210987654321"); 
        let _ = contract.constructor(logic, admin);
        
        // Attempt to change admin as a non-admin msg sender
        let result = contract.change_admin(address!("0x0987654321098765432109876543210987654321"));
        assert!(result.is_err());
        assert!(matches!(result, Err(ProxyErrors::Unauthorized(_))));
    }
}