# Bootcamp Verifying Lambchain

This repository contains a WIP Cosmos-SDK/CometBFT node implementation of a zkSNARK verifier,
built in Go with Ignite.

The application will interact with zkSNARK verifiers through FFI.

## Requirements

- Go
- Ignite

## Single Node Usage

To run a single node blockchain, run:

```sh
ignite chain serve
```

To verify a proof, run:

```sh
lambchaind tx lambchain verify <proof> --from alice --chain-id lambchain 
```

The proof is verified if it doesn't contain the word 'invalid'. To get the
transaction result, run:

```sh
lambchaind query tx <txhash>
```

## Simulate Production in Docker

Build docker image for full node

```sh
docker build . --label  lambchaind
```
