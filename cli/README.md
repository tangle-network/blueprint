# Tangle CLI

Create and Deploy blueprints on Tangle Network.

## Table of Contents

- [Tangle CLI](#tangle-cli)
  - [Table of Contents](#table-of-contents)
  - [Overview](#overview)
  - [Installation](#installation)
    - [Feature flags](#feature-flags)
  - [Creating a New Blueprint](#creating-a-new-blueprint)
    - [Example](#example)
  - [Build The Blueprint](#build-the-blueprint)
  - [Unit Testing](#unit-testing)
  - [Deploying the Blueprint to a Local Tangle Node](#deploying-the-blueprint-to-a-local-tangle-node)
    - [Example](#example-1)
  - [Required Environment Variables for Deployment](#required-environment-variables-for-deployment)
    - [Example of ENV Variables](#example-of-env-variables)
  - [Generating Keys from the Command Line](#generating-keys-from-the-command-line)
    - [Flags](#flags)

## Overview

The Tangle CLI is a command-line tool that allows you to create and deploy blueprints on the Tangle network. It
provides a simple and efficient way to manage your blueprints, making it easy to get started with Tangle
Blueprints.

## Installation

To install the Tangle CLI, run the following command:

> Supported on Linux, MacOS, and Windows (WSL2)

```bash
cargo install cargo-tangle --git https://github.com/tangle-network/blueprint --force
```

## Creating a New Tangle Blueprint

To create a new blueprint using the Tangle CLI, use the following command:

```bash
cargo tangle blueprint create --name <blueprint_name>
```

Replace `<blueprint_name>` with the desired name for your blueprint.

### Example

```bash
cargo tangle blueprint create --name my_blueprint
```

## Build The Blueprint

To build the blueprint, you can simply use cargo as you would with any rust project:

```bash
cargo build
```

## Unit Testing

To run the unit tests, use the following command:

```bash
cargo test
```

## Deploying the Blueprint to a Local Tangle Node

To deploy the blueprint to a local Tangle node, use the following command:

```bash
cargo tangle blueprint deploy tangle --devnet --package <package_name>
```

Replace `<package_name>` with the name of the package to deploy, or it can be omitted if the workspace has only one package. Using the devnet flag automatically starts a local Tangle testnet
and creates a keystore for you. The deployment keystore is generated at `./deploy-keystore` with Bob's account keys. Additionally, it generates a second keystore for testing purposes at `./test-keystore` with Alice's account keys.

### Example

```bash
cargo tangle blueprint deploy tangle --ws-rpc-url ws://localhost:9944 --keystore-path ./my-keystore --package my_blueprint
```

Expected output:

```
Blueprint #0 created successfully by 5F3sa2TJAWMqDhXG6jhV4N8ko9rUjC2q7z6z5V5s5V5s5V5s with extrinsic hash: 0x1234567890abcdef
```

## Optional Environment Variables for Deployment

The following environment variables are optional for deploying the blueprint:

- `SIGNER`: The SURI of the signer account.
- `EVM_SIGNER`: The SURI of the EVM signer account.

These environment variables can be specified instead of supplying a keystore when deploying a blueprint. It should be noted that these environment variables are not prioritized over a supplied keystore.

### Example of ENV Variables

```bash
export SIGNER="//Alice" # Substrate Signer account
export EVM_SIGNER="0xcb6df9de1efca7a3998a8ead4e02159d5fa99c3e0d4fd6432667390bb4726854" # EVM signer account
```

## Interacting with a deployed Blueprint

Once the blueprint is deployed, it can now be used on-chain. We have a collection of CLI commands that are useful for interacting with Blueprints, including the ones covered above:

- Create: `cargo tangle blueprint create`
- Deploy: `cargo tangle blueprint deploy`
- List Service Requests: `cargo tangle blueprint list-requests`
- List Blueprints: `cargo tangle blueprint list-blueprints`
- Register: `cargo tangle blueprint register`
- Request Service: `cargo tangle blueprint request-service`
- Accept Service Request: `cargo tangle blueprint accept`
- Reject Service Request: `cargo tangle blueprint reject`
- Run Blueprint: `cargo tangle blueprint run`
- Submit Job: `cargo tangle blueprint submit`

Further details on each command, as well as a full demo, can be found on our [Tangle CLI docs page](https://docs.tangle.tools/developers/cli/tangle).

## Generating Keys from the Command Line

The following command will generate a keypair for a given key type:

```shell
cargo tangle blueprint generate-keys -k <KEY_TYPE> -p <PATH> -s <SURI/SEED> --show-secret
```

where it is optional to include the path, seed, or the show-secret flags.

### Flags

- `-k` or `--key-type`: Required flag. The key type to generate (sr25519, ecdsa, bls_bn254, ed25519, bls381).
- `-p` or `--path`: The path to write the generated keypair to. If not provided, the keypair will be written solely to stdout.
- `-s` or `--seed`: The suri/seed to generate the keypair from. If not provided, a random keypair will be generated.
- `--show-secret`: Denotes that the Private Key should also be printed to stdout. If not provided, only the public key will be printed.

For all of our features for created and using keys, see the [key management](https://docs.tangle.tools/developers/cli/keys) section of our CLI docs.
