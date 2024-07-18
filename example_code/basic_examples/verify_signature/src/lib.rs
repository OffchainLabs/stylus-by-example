#![no_main]
#![no_std]
extern crate alloc;

/// Use an efficient WASM allocator.
#[global_allocator]
static ALLOC: mini_alloc::MiniAlloc = mini_alloc::MiniAlloc::INIT;

use alloy_primitives::FixedBytes;
/// Import items from the SDK. The prelude contains common traits and macros.
use stylus_sdk::{abi::Bytes, alloy_primitives::{address, Address, U256}, call::{self, Call}, prelude::*};
use alloc::string::String;
use alloy_sol_types::{sol_data::{Address as SOLAddress, FixedBytes as SolFixedBytes, *}, SolType, sol};
use sha3::{Digest, Keccak256};

type ECRECOVERType = (SolFixedBytes<32>, Uint<8>, SolFixedBytes<32>, SolFixedBytes<32>);

sol!{
    error EcrecoverCallError();
    error InvalidSignatureLength();
}

// Define some persistent storage using the Solidity ABI.
// `VerifySignature` will be the entrypoint.
sol_storage! {
    #[entrypoint]
    pub struct VerifySignature {}
}

#[derive(SolidityError)]
pub enum VerifySignatureError {
    EcrecoverCallError(EcrecoverCallError),
    InvalidSignatureLength(InvalidSignatureLength),
}

const ECRECOVER: Address = address!("0000000000000000000000000000000000000001");
const SIGNED_MESSAGE_HEAD: &'static str = "\x19Ethereum Signed Message:\n32";
// Declare private method.
impl VerifySignature {
    fn keccak256(&self, data: Bytes) -> FixedBytes<32> {
        // prepare hasher
        let mut hasher = Keccak256::new();
        // populate the data
        hasher.update(data);
        // hashing with keccack256
        let result = hasher.finalize();
        // convert the result hash to FixedBytes<32>
        let result_vec = result.to_vec();
        FixedBytes::<32>::from_slice(&result_vec)   
    }
}

/// Declare that `VerifySignature` is a contract with the following external methods.
#[external]
impl VerifySignature {
    /* 1. Unlock MetaMask account
    ethereum.enable()
    */

    /* 2. Get message hash to sign
    getMessageHash(
        0x14723A09ACff6D2A60DcdF7aA4AFf308FDDC160C,
        123,
        "coffee and donuts",
        1
    )

    hash = "0xcf36ac4f97dc10d91fc2cbb20d718e94a8cbfe0f82eaedc6a4aa38946fb797cd"
    */
    pub fn get_message_hash(
        &self,
        to: Address,
        amount: U256,
        message: String,
        nonce: U256,
    ) -> FixedBytes<32> {
        let message_data = [&to.to_vec(), &amount.to_be_bytes_vec(), message.as_bytes(), &nonce.to_be_bytes_vec()].concat();
        self.keccak256(message_data.into())
    }

    /* 3. Sign message hash
    # using browser
    account = "copy paste account of signer here"
    ethereum.request({ method: "personal_sign", params: [account, hash]}).then(console.log)

    # using web3
    web3.personal.sign(hash, web3.eth.defaultAccount, console.log)

    Signature will be different for different accounts
    0x993dab3dd91f5c6dc28e17439be475478f5635c92a56e17e82349d3fb2f166196f466c0b4e0c146f285204f0dcb13e5ae67bc33f4b888ec32dfe0a063e8f3f781b
    */
    pub fn get_eth_signed_message_hash(&self, message_hash: FixedBytes<32>) -> FixedBytes<32> {
        let message_to_be_decoded = [SIGNED_MESSAGE_HEAD.as_bytes(), &message_hash.to_vec()].concat();
        self.keccak256(message_to_be_decoded.into())
    }

    /* 4. Verify signature
    signer = 0xB273216C05A8c0D4F0a4Dd0d7Bae1D2EfFE636dd
    to = 0x14723A09ACff6D2A60DcdF7aA4AFf308FDDC160C
    amount = 123
    message = "coffee and donuts"
    nonce = 1
    signature =
        0x993dab3dd91f5c6dc28e17439be475478f5635c92a56e17e82349d3fb2f166196f466c0b4e0c146f285204f0dcb13e5ae67bc33f4b888ec32dfe0a063e8f3f781b
    */
    pub fn verify(
        &self,
        signer: Address,
        to: Address,
        amount: U256,
        message: String,
        nonce: U256,
        signature: Bytes,
    ) -> Result<bool, VerifySignatureError> {
        let message_hash = self.get_message_hash(to, amount, message, nonce);
        let eth_signed_message_hash = self.get_eth_signed_message_hash(message_hash);
        match self.recover_signer(eth_signed_message_hash, signature) {
            Ok(recovered_signer) => Ok(recovered_signer == signer),
            Err(err) => Err(err),
        }
    }

    pub fn recover_signer(
        &self,
        eth_signed_message_hash: FixedBytes<32>,
        signature: Bytes
    ) -> Result<Address, VerifySignatureError> {
        let (r, s, v) = self.split_signature(signature);
        self.ecrecover(eth_signed_message_hash, v, r, s)
    }

    /// Invoke the ECRECOVER precompile.
    pub fn ecrecover(
        &self,
        hash: FixedBytes<32>,
        v: u8,
        r: FixedBytes<32>,
        s: FixedBytes<32>,
    ) -> Result<Address, VerifySignatureError> {
        let data = (hash, v, r, s);
        let encoded_data = ECRECOVERType::abi_encode(&data);
        match call::static_call(Call::new(), ECRECOVER, &encoded_data) {
            Ok(result) => Ok(SOLAddress::abi_decode(&result, false).unwrap()),
            Err(_) => Err(VerifySignatureError::EcrecoverCallError(EcrecoverCallError{})),
        }
    }


    pub fn split_signature(
        &self,
        signature: Bytes
    ) -> (FixedBytes<32>, FixedBytes<32>, u8) {
        let r = FixedBytes::from_slice(&signature[0..32]);
        let s = FixedBytes::from_slice(&signature[32..64]);
        let v = signature[64];
        (r, s, v)
    }
            
}
