#[cfg(feature = "export-abi")]
fn main() {
    stylus_erc721::print_abi("MIT-OR-APACHE-2.0", "pragma solidity ^0.8.23;");
}