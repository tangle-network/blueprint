## Incredible Squaring Blueprint for Eigenlayer

A simple AVS blueprint that only has one job - taking **x** and signing **x<sup>2</sup>**, and then aggregating the BLS signatures and submitting it onchain.

## Prerequisites

Before you begin, ensure you have the following installed:

- [Anvil](https://book.getfoundry.sh/anvil/)
- [Docker](https://www.docker.com/get-started)

## Installation

1. Clone this repository:
   ```bash
   git clone https://github.com/tangle-network/blueprint.git
   cd blueprint
   ```
   
2. Install Anvil:
   ```bash
   curl -L https://foundry.paradigm.xyz | bash
   foundryup
   ```

## Building the Blueprint

- To build the blueprint, run the following command:

```bash
cargo build --release -p incredible-squaring-blueprint-eigenlayer
```

## Running the test

- We have a test for running this AVS Blueprint on a local Anvil Testnet. You can run the test with the following:

```bash
RUST_LOG=incredible_squaring_blueprint_eigenlayer=trace cargo test --package incredible-squaring-blueprint-eigenlayer test_eigenlayer_incredible_squaring_blueprint -- --nocapture
```

## Deploy to localnet

- Build the source:
```sh
cargo build --release
```

- Deploy the source in this ordering:
   - SquaringTask
   - SquaringServiceManager

```sh
cargo tangle blueprint deploy eigenlayer \
  --devnet \
  --ordered-deployment
```

- Initialize `SquaringTask`
```sh
# Read more at examples/incredible-squaring-eigenlayer/src/lib.rs to know about these values
#  function initialize(
#     address _aggregator,
#     address _generator,
#     address initialOwner
#  )
cast send 0xc0f115a19107322cfbf1cdbc7ea011c19ebdb4f8 "initialize(address,address,address)" \
  0xa0Ee7A142d267C1f36714E4a8F75612F20a79720 \
  0x15d34AAf54267DB7D7c367839AAf71A00a2C6A65 \
  0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266 \
  --private-key 0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80
```

- Run the Blueprint:
```sh
RUST_LOG=info cargo tangle blueprint run \
     -p eigenlayer \
     -u http://localhost:55000/ \
     --keystore-path ./test-keystore \
     -b ../../target/release/incredible-squaring-blueprint-eigenlayer \
     -f settings.env
```