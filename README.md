# Near Vote

This is my submission for the Near Certified Developer project, a private, trustless voting contract.

## Description

This project is motivated by a future world where everyone has a unique NEAR account linked to their government identification, and political votes can be performed using a secure and decentralised manner. There are of course still huge challenges to this, however, if only a single vote contract is known to be the one true source of information, it could serve as a far better solution than anything using a fully centralized pattern.

The contract in this repository is defined by the following criteria:

1. The government (or anyone) can instantiate a new Vote contract with a given set of voting options as strings, and a given timestamp (Unix nanoseconds) at which the vote will end.
2. Once a vote contract has been intialised, the voting options are immutable, and a new contract must be deployed if a new vote is required, this is by design to prevent tampering with voting options or votes that are ongoing, and also to ensure that there can only be *one true vote contract ID*, all others cannot be trusted.
3. A vote can only be issued once per user. No wallet can vote twice.
4. A user can only see their own votes, not anyone elses. In order to do so, they must actually sign a transaction so we can validate that they are who they say they are.
5. Once the vote has ended, anyone can see the final results of the vote.

## Usage

### Getting started

(see below for video recordings of each of the following steps)

INSTALL `NEAR CLI` first like this: `npm i -g near-cli`
INSTALL RUST toolchain
Add the wasm target using `rustup target add wasm32-unknown-unknown`

You will need to have authenticated with the near wallet in order to deploy the contract, see here:

https://docs.near.org/docs/tools/near-cli#near-login

You will also need to set your "ACCOUNT" environment variable.

1. clone this repo to a local folder
2. run `./build.sh` to build the wasm file
2. run `. ./scripts/1.dev-deploy.sh` to deploy to testnet. Remember to set $CONTRACT environment variable.
3. run `./scripts/2.use-contract.sh` to interact with the contract.
4. run `./scripts/3.cleanup.sh` to delete the contract.

Created with love by lovathom.near ❤️
