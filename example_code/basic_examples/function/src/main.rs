#[cfg(feature = "export-abi")]
fn main() {
    stylus_functions::print_abi("MIT-OR-APACHE-2.0", "pragma solidity ^0.8.23;");
}