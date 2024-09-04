
#![cfg_attr(not(any(feature = "export-abi", test)), no_main)]
extern crate alloc;


/// Import items from the SDK. The prelude contains common traits and macros.
use stylus_sdk::{alloy_primitives::{U256, Address}, prelude::*};
// Because the naming of `alloy_primitives` and `alloy_sol_types` is the same, we need to rename the types in `alloy_sol_types`.
use alloy_sol_types::{sol_data::{Address as SOLAddress, *}, SolType, sol};


// Define error
sol! {
    error DecodedFailed();
}

// Error types for the MultiSig contract
#[derive(SolidityError)]
pub enum DecoderError{
    DecodedFailed(DecodedFailed)
}

// Define some persistent storage using the Solidity ABI.
// `Decoder` will be the entrypoint.
sol_storage! {
    #[entrypoint]
    pub struct Decoder {
    }
}


/// Declare that `Decoder` is a contract with the following external methods.
#[public]
impl Decoder {
    // This should always return true
    pub fn encode_and_decode(
        &self, 
        address: Address, 
        amount: U256
    ) -> Result<bool, DecoderError> {
        // define sol types tuple
        type TxIdHashType = (SOLAddress, Uint<256>);
        // set the tuple
        let tx_hash_data = (address, amount);
        // encode the tuple
        let tx_hash_data_encode = TxIdHashType::abi_encode_sequence(&tx_hash_data);

        let validate = true;
        
        // Check the result
        match TxIdHashType::abi_decode_sequence(&tx_hash_data_encode, validate) {
            Ok(res) => Ok(res == tx_hash_data),
            Err(_) => {
                return Err(DecoderError::DecodedFailed(DecodedFailed{}));
            },
        }   
    }

}