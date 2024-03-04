# Bootcamp Verifying Blockchain

This repository contains a WIP CometBFT ABCI implementation of a zkSNARK verifier,
built in Rust.

## Single Node Usage

Requires Rust and CometBFT to run.

Run the ABCI application in a terminal:

```bash
make application
```

To run a liar node (accepts every transaction), run:
```bash
make liar
```

Run a single CometBFT node:

```bash
make node
```

The CometBFT node exposes HTTP endpoints to operate with the blockchain. They
can be visualized at [localhost:26657](http://localhost:26657).

To clear CometBFT configuration, run:

```bash
make clear
```
