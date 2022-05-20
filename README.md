# `near-sdk-rs` Starter Kit

This is a good project to use as a starting point for your Rust project.

## Samples

This repository includes a very basic project structure for Rust contracts targeting the NEAR platform.

The example here is a simple contract demonstrating the following concepts:
- a single contract
- the difference between `view` vs. `change` methods

There is 1 Rust contract in this project:

- **status message** in the `src` folder

## Usage

### Getting started

(see below for video recordings of each of the following steps)

INSTALL `NEAR CLI` first like this: `npm i -g near-cli`
INSTALL RUST toolchain
Add the wasm target using `rustup target add wasm32-unknown-unknown`

1. clone this repo to a local folder
2. run `./scripts/1.dev-deploy.sh`
3. run `./scripts/2.use-contract.sh`
4. run `./scripts/3.cleanup.sh`
