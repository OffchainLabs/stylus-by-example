# stylus-by-example

Welcome to Stylus by Example! This repository contains a collection of concise, runnable examples that demonstrate core concepts and common patterns for writing smart contracts with [Arbitrum Stylus](https://docs.arbitrum.io/stylus/introduction). Whether you're new to Stylus or looking to deepen your understanding, these examples are designed to be a quick and accessible resource.

## Structure

The repository is organized into two main categories of examples:

- **`example_code/external/`**: These are direct copies of examples from the [stylus-sdk-rs](https://github.com/OffchainLabs/stylus-sdk-rs) repository. They are kept up-to-date automatically by a GitHub Actions workflow that copies the latest versions and updates their `Cargo.toml` dependencies to use the latest `stylus-sdk` and `stylus-tools` from [crates.io](https://crates.io).

- **`example_code/internal/`**: These are custom examples developed specifically for this repository, often showcasing more complex applications or unique patterns not covered by the external examples.

## Getting Started

**1. Clone the repository:**

```bash
git clone https://github.com/OffchainLabs/stylus-by-example
```

**2. Explore the examples:**

Navigate to `example_code/external/` or `example_code/internal/` to browse the available examples. Each example is a self-contained Rust project with its own `Cargo.toml`.

**3. Build and test:**

To build and test an example, navigate into its directory and run `cargo build` and `cargo test`:

```bash
cd example_code/external/hello_world
cargo build
cargo test
```

## Contributions

We welcome contributions to Stylus by Example! If you have a new example idea, an improvement to an existing one, or a bug fix, please feel free to open a pull request.

## License

This project is licensed under the MIT License.