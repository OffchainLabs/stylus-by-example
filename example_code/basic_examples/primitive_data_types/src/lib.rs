#![cfg_attr(not(any(feature = "export-abi", test)), no_main)]
extern crate alloc;

#[global_allocator]
static ALLOC: mini_alloc::MiniAlloc = mini_alloc::MiniAlloc::INIT;
use alloc::{string::ToString, vec::Vec};

use stylus_sdk::{
    alloy_primitives::{address, Address, I256, I8, U256, U8}, console, prelude::*, ArbResult
};

#[solidity_storage]
#[entrypoint]
pub struct Events {
    
}


#[external]
impl Events {
fn user_main(_input: Vec<u8>) -> ArbResult {
    // Use native Rust primitives where they make sense
    // and where no equivalent Alloy primitive exists
    let frightened: bool = true;
    // Out: Stylus says: 'Boo! Did I scare you?'
    console!("Boo! Did I scare you?");

    let response = match frightened {
        true => "Yes!".to_string(),
        false => "No!".to_string(),
    };

    // Out: Stylus says: 'Yes!'
    console!("{response}");

    // U256 stands for a 256-bit *unsigned* integer, meaning it cannot be
    // negative. The range for a U256 number is 0 to 2^256 - 1. Alloy provides
    // a set of unsigned integer types to represent the various sizes available
    // in the EVM.
    //    U256 maps to uint256
    //    U128 maps to uint128
    //    ...
    //    U8 maps to uint8
    let eight_bit: U8 = U8::from(1);
    let two_fifty_six_bit: U256 = U256::from(0xff_u64);

    // Out: Stylus says: '8-bit: 1 | 256-bit: 255'
    console!("8-bit: {} | 256-bit: {}", eight_bit, two_fifty_six_bit);

    // Negative numbers are allowed for I types. These represent signed integers.
    //    I256 maps to int256
    //    I128 maps to int128
    //    ...
    //    I8 maps to int8
    let eight_bit: I8 = I8::unchecked_from(-1);
    let two_fifty_six_bit: I256 = I256::unchecked_from(0xff_u64);

    // Out: Stylus says: '8-bit: -1 | 256-bit: 255'
    console!("8-bit: {} | 256-bit: {}", eight_bit, two_fifty_six_bit);

    // Additional usage of integers

    // Use `try_from` if you're not sure it'll fit
    let a = I256::try_from(20003000).unwrap();
    // Or parse from a string
    let b = "100".parse::<I256>().unwrap();
    // With hex characters
    let c = "-0x138f".parse::<I256>().unwrap();
    // Underscores are ignored
    let d = "1_000_000".parse::<I256>().unwrap();

    // Math works great
    let e = a * b + c - d;
    // Out: Stylus says: '20003000 * 100 + -5007 - 1000000 = 1999294993'
    console!("{} * {} + {} - {} = {}", a, b, c, d, e);

    // Useful constants
    let f = I256::MAX;
    let g = I256::MIN;
    let h = I256::ZERO;
    let i = I256::MINUS_ONE;

    // Stylus says: '5789...9967, -5789...9968, 0, -1'
    console!("{f}, {g}, {h}, {i}");
    // As hex: Stylus says: '0x7fff...ffff, 0x8000...0000, 0x0, 0xffff...ffff'
    console!("{:#x}, {:#x}, {:#x}, {:#x}", f, g, h, i);

    // Ethereum addresses are 20 bytes in length, or 160 bits. Alloy provides a number of helper utilities for converting to addresses from strings, bytes, numbers, and addresses

    // From a 20 byte slice, all 1s
    let addr1 = Address::from([0x11; 20]);
    // Out: Stylus says: '0x1111111111111111111111111111111111111111'
    console!("{addr1}");

    // Use the address! macro to parse a string as a checksummed address
    let addr2 = address!("d8da6bf26964af9d7eed9e03e53415d37aa96045");
    // Out: Stylus says: '0xd8dA6BF26964aF9D7eEd9e03E53415D37aA96045'
    console!("{addr2}");

    // Format compressed addresses for output
    // Out: Stylus says: '0xd8dA…6045'
    console!("{addr2:#}");

    Ok(Vec::new())
}
}