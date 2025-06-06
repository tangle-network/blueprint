## Incredible Squaring Blueprint

A simple blueprint that only has one job that takes **x** and returns **x<sup>2</sup>**.

## Building the Blueprint

- To build the blueprint, just run the following command:

```bash
cargo build -p incredible-squaring-blueprint
```

## Running the Blueprint

- To run the blueprint on Local Tangle Network, make sure you have tangle running on your local machine first by running
  the following command:

```bash
bash ./scripts/run-standalone-local.sh --clean
```

- Add Alice to your local keystore, so that the blueprint can use it to sign transactions:

```bash
 echo -n "e5be9a5092b81bca64be81d212e7f2f9eba183bb7a90954f7b76361f6edb5c0a" > target/keystore/0000d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d
```

- Then, use the following command to run the blueprint that uses the blueprint:

```bash
RUST_LOG=blueprint_sdk=trace,error cargo run -p incredible-squaring-blueprint -- run --url=ws://localhost:9944 --base-path=./target/keystore --blueprint-id=0 --service-id=0 --target-addr=0.0.0.0 --target-port=<bind-port>
```

That's it! You should see the blueprint running and listening for incoming requests on the Local Tangle Network.

## Deploying the Blueprint to the Tangle

- To deploy the blueprint to the tangle, make sure you have tangle running on your local machine first by running the
  following command:

```bash
bash ./scripts/run-standalone-local.sh --clean
```

- (Optionally) Visit [Polkadot JS UI](https://polkadot.js.org/apps/?rpc=ws://127.0.0.1:9944#/explorer) to check the
  status of the tangle network.
- Install the required dependencies by running the following command:

```bash
cd blueprints/incredible-squaring && yarn install
```

- Compile the Contracts using forge command:

```bash
forge build --root ./contracts
```

- Next, run the following command to deploy the blueprint to the tangle:

```bash
yarn tsx deploy.ts
```

This script will deploy the blueprint to the tangle network, and create a service instance that you can use to interact
with the blueprint, then submit a request to the service instance to square a number.
