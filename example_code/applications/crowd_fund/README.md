# Stylus Crowd Fund Example

An example project for writing Arbitrum Stylus programs in Rust using the [stylus-sdk](https://github.com/OffchainLabs/stylus-sdk-rs). It includes a Rust implementation of a crowd fund Ethereum smart contract. Below is the interface for the CrowdFund contract:

```js
// SPDX-License-Identifier: MIT-OR-APACHE-2.0
pragma solidity ^0.8.23;

interface ICrowdFund {
    function launch(uint256 goal, uint256 start_at, uint256 end_at) external;

    function cancel(uint256 id) external;

    function pledge(uint256 id, uint256 amount) external;

    function unpledge(uint256 id, uint256 amount) external;

    function claim(uint256 id) external;

    function refund(uint256 id) external;
}
```

### ABI Export

You can export the Solidity ABI for your program by using the `cargo stylus` tool as follows:

```bash
cargo stylus export-abi
```

## Deploying

You can use the `cargo stylus` command to also deploy your program to the Stylus testnet. We can use the tool to first check
our program compiles to valid WASM for Stylus and will succeed a deployment onchain without transacting. By default, this will use the Stylus testnet public RPC endpoint. See here for [Stylus testnet information](https://docs.arbitrum.io/stylus/reference/testnet-information).

```bash
cargo stylus check
```

Next, we deploy:

```bash
cargo stylus deploy \
  --private-key-path=<PRIVKEY_FILE_PATH>
```
