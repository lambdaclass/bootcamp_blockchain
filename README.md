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
docker build . -t lambchaind_i
```

Initialize each node with the same command

```sh
xargs -I {} \
    docker run --rm -i \
    -v $(pwd)/prod-sim/{}:/root/.lambchain \
    lambchaind_i \
    init lambchain \
<< EOF
alice
bob
carol
EOF
```
