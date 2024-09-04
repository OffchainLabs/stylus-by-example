// Allow `cargo stylus export-abi` to generate a main function.
#![cfg_attr(not(feature = "export-abi"), no_main)]
extern crate alloc;

/// Import items from the SDK. The prelude contains common traits and macros.
use stylus_sdk::{alloy_primitives::{Address, U256}, prelude::*, msg};

// Define some persistent storage using the Solidity ABI.
// `Counter` will be the entrypoint.
sol_storage! {
    #[entrypoint]
    pub struct Counter {
        uint256[] arr;
        uint256[10] arr2; // fixed length array
        Info[] arr3; // struct array
    }

    pub struct Info {
        address setter;
        uint256 value;
    }
}

/// Declare that `Counter` is a contract with the following external methods.
#[public]
impl Counter {
    pub fn push(&mut self, i: U256) {
        self.arr.push(i);
    }


    pub fn get_element(&self, index: U256) -> U256 {
        self.arr.get(index).unwrap()
    }

    pub fn get_arr_length(&self) -> U256 {
        U256::from(self.arr.len())
    }

    pub fn remove(&mut self, index: U256) {
        let mut last_element = self.arr.setter(index).unwrap();
        last_element.erase()
    }

    // fixed length array
    pub fn get_arr2_size(&self) -> U256 {
        U256::from(self.arr2.len())
    }

    pub fn get_arr2_element(&self, index: U256) -> U256 {
        self.arr2.get(index).unwrap()
    }

    pub fn set_arr2_value(&mut self, index: U256, value: U256) {
        self.arr2.setter(index).unwrap().set(value);
    }

    // struct array
    pub fn push_arr3_info(&mut self, value: U256) {
        let mut new_info = self.arr3.grow();
        new_info.setter.set(msg::sender());
        new_info.value.set(value);
    }

    pub fn get_arr3_length(&self) -> U256 {
        U256::from(self.arr3.len())
    }

    pub fn get_arr3_info(&self, index: U256) -> (Address, U256) {
        let info = self.arr3.get(index).unwrap();
        (info.setter.get(), info.value.get())
    }

    // Find the first index of the expected value in the array
    pub fn find_arr3_first_expected_value(&self, expected_value: U256) -> U256 {
        for i in 0..self.arr3.len() {
            let (_, value) = self.get_arr3_info(U256::from(i));
            if value == expected_value {
                return U256::from(i);
            }
        }
        // if not found, return the size of arr
        U256::from(self.arr3.len())
    }
}
