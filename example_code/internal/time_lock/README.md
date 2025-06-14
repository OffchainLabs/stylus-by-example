# Stylus TimeLock Example

Project starter template for writing Arbitrum Stylus programs in Rust using the [stylus-sdk](https://github.com/OffchainLabs/stylus-sdk-rs). It includes a Rust implementation of a basic Time Lock smart contract:

```js
// SPDX-License-Identifier: MIT-OR-APACHE-2.0
pragma solidity ^0.8.23;

interface ITimeLock {
    function initialize() external;

    function owner() external view returns (address);

    function getTxId(address target, uint256 value, string calldata func, bytes calldata data, uint256 timestamp) external view returns (bytes32);

    function deposit() external payable;

    function queue(address target, uint256 value, string calldata func, bytes calldata data, uint256 timestamp) external;

    function execute(address target, uint256 value, string calldata func, bytes calldata data, uint256 timestamp) external;

    function cancel(address target, uint256 value, string calldata func, bytes calldata data, uint256 timestamp) external;

    error AlreadyInitialized();

    error NotOwnerError();

    error AlreadyQueuedError(bytes32);

    error TimestampNotInRangeError(uint256, uint256);

    error NotQueuedError(bytes32);

    error TimestampNotPassedError(uint256, uint256);

    error TimestampExpiredError(uint256, uint256);

    error TxFailedError();
}
```

### ABI Export

You can export the Solidity ABI for your program by using the `cargo stylus` tool as follows:

```bash
cargo stylus export-abi
```

which outputs the same interface as above.

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
