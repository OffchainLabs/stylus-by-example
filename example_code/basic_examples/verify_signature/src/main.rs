#[cfg(feature = "export-abi")]
fn main() {
    stylus_verify_signature::print_abi("MIT-OR-APACHE-2.0", "pragma solidity ^0.8.23;");
}