# Stylus English Auction Example

An example project for writing Arbitrum Stylus programs in Rust using the [stylus-sdk](https://github.com/OffchainLabs/stylus-sdk-rs). It includes a Rust implementation of a vending machine Ethereum smart contract. Below is the interface for the EnglishAuction contract:

```js
// SPDX-License-Identifier: MIT-OR-APACHE-2.0
pragma solidity ^0.8.23;

interface IEnglishAuction {
    function nft() external view returns (address);

    function nftId() external view returns (uint256);

    function seller() external view returns (address);

    function endAt() external view returns (uint256);

    function started() external view returns (bool);

    function ended() external view returns (bool);

    function highestBidder() external view returns (address);

    function highestBid() external view returns (uint256);

    function bids(address bidder) external view returns (uint256);

    function initialize(address nft, uint256 nft_id, uint256 starting_bid) external;

    function start() external;

    function bid() external payable;

    function withdraw() external;

    function end() external;

    error AlreadyInitialized();

    error AlreadyStarted();

    error NotSeller();

    error AuctionEnded();

    error BidTooLow();

    error NotStarted();

    error NotEnded();
}
```

### ABI Export

You can export the Solidity ABI for your program by using the `cargo stylus` tool as follows:

```bash
cargo stylus export-abi
```

which outputs:

```js
// SPDX-License-Identifier: MIT-OR-APACHE-2.0
pragma solidity ^0.8.23;

interface IEnglishAuction {
    function nft() external view returns (address);

    function nftId() external view returns (uint256);

    function seller() external view returns (address);

    function endAt() external view returns (uint256);

    function started() external view returns (bool);

    function ended() external view returns (bool);

    function highestBidder() external view returns (address);

    function highestBid() external view returns (uint256);

    function bids(address bidder) external view returns (uint256);

    function initialize(address nft, uint256 nft_id, uint256 starting_bid) external;

    function start() external;

    function bid() external payable;

    function withdraw() external;

    function end() external;

    error AlreadyInitialized();

    error AlreadyStarted();

    error NotSeller();

    error AuctionEnded();

    error BidTooLow();

    error NotStarted();

    error NotEnded();
}
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
