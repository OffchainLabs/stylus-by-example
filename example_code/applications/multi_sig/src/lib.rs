// Allow `cargo stylus export-abi` to generate a main function.
#![cfg_attr(not(feature = "export-abi"), no_main)]
extern crate alloc;

/// Import items from the SDK. The prelude contains common traits and macros.
use stylus_sdk::{contract, evm, msg, prelude::*, call::{Call, call}, alloy_primitives::{Address, U256}, abi::Bytes};
use alloy_sol_types::sol;

// Define some events using the Solidity ABI.
sol! {
    event Deposit(address indexed sender, uint256 amount, uint256 balance);
    event SubmitTransaction(
        address indexed owner,
        uint256 indexed txIndex,
        address indexed to,
        uint256 value,
        bytes data
    );
    event ConfirmTransaction(address indexed owner, uint256 indexed txIndex);
    event RevokeConfirmation(address indexed owner, uint256 indexed txIndex);
    event ExecuteTransaction(address indexed owner, uint256 indexed txIndex);

    // Error types for the MultiSig contract
    error AlreadyInitialized();
    error ZeroOwners(); // The owners number is 0 when init.
    error InvaildConfirmationNumber();
    error InvalidOwner(); // The owner address is invalid when init.
    error OwnerNotUnique(); // The owner address is not unique when init.
    error NotOwner(); // The sender is not an owner.
    error TxDoesNotExist();
    error TxAlreadyExecuted();
    error TxAlreadyConfirmed();
    error TxNotConfirmed();
    error ConfirmationNumberNotEnough();
    error ExecuteFailed();
}

// Define some persistent storage using the Solidity ABI.
// `MultiSig` will be the entrypoint.
sol_storage! {
    #[entrypoint]
    pub struct MultiSig {
        address[] owners; // The addresses of the owners
        mapping(address => bool) is_owner; // mapping from owner => bool
        uint256 num_confirmations_required; // The number of confirmations required to execute a transaction
        TxStruct[] transactions; // The transactions array
        // mapping from tx index => owner => bool
        mapping(uint256 => mapping(address => bool)) is_confirmed;
    }

    // Define the `TxStruct` struct
    pub struct TxStruct {
        address to;
        uint256 value;
        bytes data;
        bool executed; // Whether the transaction has been executed
        uint256 num_confirmations; // The number of confirmations of the current transaction
    }
}

// Error types for the MultiSig contract
#[derive(SolidityError)]
pub enum MultiSigError {
    AlreadyInitialized(AlreadyInitialized),
    ZeroOwners(ZeroOwners),
    InvaildConfirmationNumber(InvaildConfirmationNumber),
    InvalidOwner(InvalidOwner),
    OwnerNotUnique(OwnerNotUnique),
    NotOwner(NotOwner),
    TxDoesNotExist(TxDoesNotExist),
    TxAlreadyExecuted(TxAlreadyExecuted),
    TxAlreadyConfirmed(TxAlreadyConfirmed),
    TxNotConfirmed(TxNotConfirmed),
    ConfirmationNumberNotEnough(ConfirmationNumberNotEnough),
    ExecuteFailed(ExecuteFailed),
}

/// Declare that `MultiSig` is a contract with the following external methods.
#[public]
impl MultiSig {
    pub fn num_confirmations_required(&self) -> Result<U256, MultiSigError> {
        Ok(self.num_confirmations_required.get())
    }

    // The `deposit` method is payable, so it can receive funds.
    #[payable]
    pub fn deposit(&mut self) {
        let sender = msg::sender();
        let amount = msg::value();
        evm::log(
            Deposit{
                sender: sender, 
                amount: amount, 
                balance: contract::balance()
            });
    }

    // The `submit_transaction` method submits a new transaction to the contract.
    pub fn submit_transaction(&mut self, to: Address, value: U256, data: Bytes) -> Result<(), MultiSigError> {
        // The sender must be an owner.
        if !self.is_owner.get(msg::sender()) {
            return Err(MultiSigError::NotOwner(NotOwner{}));
        }

        let tx_index = U256::from(self.transactions.len());
        
        // Add the transaction to the transactions array.
        let mut new_tx = self.transactions.grow();
        new_tx.to.set(to);
        new_tx.value.set(value);
        new_tx.data.set_bytes(data.clone());
        new_tx.executed.set(false);
        new_tx.num_confirmations.set(U256::from(0));

        // Emit the `SubmitTransaction` event.
        evm::log(SubmitTransaction {
            owner: msg::sender(),
            txIndex: tx_index,
            to: to,
            value: value,
            data: data.to_vec().into(),
        });
        Ok(())
    }


    // The `initialize` method initializes the contract with the owners and the number of confirmations required.
    pub fn initialize(&mut self, owners: Vec<Address>, num_confirmations_required: U256) -> Result<(), MultiSigError> {
        // The owners must not be initialized.
        if self.owners.len() > 0 {
            return Err(MultiSigError::AlreadyInitialized(AlreadyInitialized{}));
        }

        // The owners must not be empty.
        if owners.len() == 0 {
            return Err(MultiSigError::ZeroOwners(ZeroOwners{}));
        }

        // The number of confirmations required must be greater than 0 and less than or equal to the number of owners.
        if num_confirmations_required == U256::from(0) || num_confirmations_required > U256::from(owners.len()) {
            return Err(MultiSigError::InvaildConfirmationNumber(InvaildConfirmationNumber{}));
        }

        // Add the owners to the contract.
        for owner in owners.iter() {
            if *owner == Address::default() {
                return Err(MultiSigError::InvalidOwner(InvalidOwner{}))
            }

            if self.is_owner.get(*owner) {
                return Err(MultiSigError::OwnerNotUnique(OwnerNotUnique{}))
            }

            self.is_owner.setter(*owner).set(true);
            self.owners.push(*owner);
        }

        // Set the number of confirmations required.
        self.num_confirmations_required.set(num_confirmations_required);
        Ok(())
    }

    // The `execute_transaction` method executes a transaction.
    pub fn execute_transaction(&mut self, tx_index: U256) -> Result<(), MultiSigError>{
        // The sender must be an owner.
        if !self.is_owner.get(msg::sender()) {
            return Err(MultiSigError::NotOwner(NotOwner{}));
        }

        // The transaction must exist.
        let tx_index = tx_index.to::<usize>();
        if tx_index >= self.transactions.len() {
            return Err(MultiSigError::TxDoesNotExist(TxDoesNotExist{}));
        }

        // Try get transaction and check transaction is valid or not, if valid, execute it, if not, revert tx.
        if let Some(mut entry) = self.transactions.get_mut(tx_index) {
            if entry.executed.get() {
                return Err(MultiSigError::TxAlreadyExecuted(TxAlreadyExecuted{}));
            }

            if entry.num_confirmations.get() < self.num_confirmations_required.get() {
                return Err(MultiSigError::ConfirmationNumberNotEnough(ConfirmationNumberNotEnough{}));
            }
            
            entry.executed.set(true);
            let entry_value = entry.value.get();
            let entry_to = entry.to.get();
            let entry_data = entry.data.get_bytes();
            // Execute the transaction
            match call(Call::new_in(self).value(entry_value), entry_to, &entry_data) {
                // If the transaction is successful, emit the `ExecuteTransaction` event.
                Ok(_) => {
                    evm::log(ExecuteTransaction {
                        owner: msg::sender(),
                        txIndex: U256::from(tx_index),
                    });
                    Ok(())
                },
                // If the transaction fails, revert the transaction.
                Err(_) => {
                    return Err(MultiSigError::ExecuteFailed(ExecuteFailed{}));
                }
            }
            
        } else {
            return Err(MultiSigError::TxDoesNotExist(TxDoesNotExist{}));
        }
    }

    // The `confirm_transaction` method confirms a transaction.
    pub fn confirm_transaction(&mut self, tx_index: U256) -> Result<(), MultiSigError> {
        // The sender must be an owner.
        if !self.is_owner.get(msg::sender()) {
            return Err(MultiSigError::NotOwner(NotOwner{}));
        }

        // The transaction must exist.
        if tx_index >= U256::from(self.transactions.len()) {
            return Err(MultiSigError::TxDoesNotExist(TxDoesNotExist{}));
        }

        // Try get transaction and check transaction is valid or not, if valid, confirm it, if not, revert tx.
        if let Some(mut entry) = self.transactions.get_mut(tx_index) {
            if entry.executed.get() {
                return Err(MultiSigError::TxAlreadyExecuted(TxAlreadyExecuted{}));
            }

            if self.is_confirmed.get(tx_index).get(msg::sender()) {
                return Err(MultiSigError::TxAlreadyConfirmed(TxAlreadyConfirmed{}));
            }

            // Confirm the transaction
            let num_confirmations = entry.num_confirmations.get();
            entry.num_confirmations.set(num_confirmations + U256::from(1));
            // Set the transaction as confirmed by the sender.
            let mut tx_confirmed_info = self.is_confirmed.setter(tx_index);
            let mut confirmed_by_address = tx_confirmed_info.setter(msg::sender());
            confirmed_by_address.set(true);

            // Emit the `ConfirmTransaction` event.
            evm::log(ConfirmTransaction {
                owner: msg::sender(),
                txIndex: U256::from(tx_index),
            });
            Ok(())
        } else {
            return Err(MultiSigError::TxDoesNotExist(TxDoesNotExist{}));
        }
    }

    // The `revoke_confirmation` method revokes a confirmation for a transaction.
    pub fn revoke_confirmation(&mut self, tx_index: U256) -> Result<(), MultiSigError> {
        // The sender must be an owner.
        if !self.is_owner.get(msg::sender()) {
            return Err(MultiSigError::NotOwner(NotOwner{}));
        }
        // let tx_index = tx_index.to;
        if tx_index >= U256::from(self.transactions.len()) {
            return Err(MultiSigError::TxDoesNotExist(TxDoesNotExist{}));
        }

        // Try get transaction and check transaction is valid or not, if valid, revoke it, if not, revert tx.
        if let Some(mut entry) = self.transactions.get_mut(tx_index) {
            // Check if the transaction has been confirmed or not
            if !self.is_confirmed.get(tx_index).get(msg::sender()) {
                // If the transaction has not been confirmed, return an error.
                return Err(MultiSigError::TxNotConfirmed(TxNotConfirmed{}));
            }

            //  Revoke the transaction
            let num_confirmations = entry.num_confirmations.get();
            entry.num_confirmations.set(num_confirmations - U256::from(1));
            // Set the transaction as not confirmed by the sender.
            let mut tx_confirmed_info = self.is_confirmed.setter(tx_index);
            let mut confirmed_by_address = tx_confirmed_info.setter(msg::sender());
            confirmed_by_address.set(false);

            //  Emit the `RevokeConfirmation` event.
            evm::log(RevokeConfirmation {
                owner: msg::sender(),
                txIndex: U256::from(tx_index),
            });
            Ok(())
        } else {
            return Err(MultiSigError::TxDoesNotExist(TxDoesNotExist{}));
        }
    }

    // The `is_owner` method checks if an address is an owner.
    pub fn is_owner(&self, check_address: Address) -> bool {
        self.is_owner.get(check_address)
    }

    // The `get_transaction_count` method returns the number of transactions.
    pub fn get_transaction_count(&self) -> U256 {
        U256::from(self.transactions.len())
    }
}