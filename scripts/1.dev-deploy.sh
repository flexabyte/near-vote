#!/usr/bin/env bash

# exit on first error after this point to avoid redeploying with successful build
set -e

echo
echo ---------------------------------------------------------
echo "Step 1: Build the contract (may take a few seconds)"
echo ---------------------------------------------------------
echo

#build for wasm target
RUSTFLAGS='-C link-arg=-s' cargo build --target wasm32-unknown-unknown --release

echo
echo
echo ---------------------------------------------------------
echo "Step 2: Deploy the contract"
echo
echo "(edit scripts/1.dev-deploy.sh to deploy other contract)"
echo ---------------------------------------------------------
echo


# comment the line below to deploy the other example contract
near dev-deploy -f ./res/near_vote.wasm
#near deploy --wasmFile res/near_vote.wasm --accountId dummy.lovanft.testnet

echo
echo
echo ---------------------------------------------------------
echo "Step 3: Prepare your environment for next steps"
echo
echo "(a) find the contract (account) name in the message above"
echo "    it will look like this: [ Account id: dev-###-### ]"
echo
echo "(b) set an environment variable using this account name"
echo "    see example below (this may not work on Windows)"
echo
echo ---------------------------------------------------------
echo "export CONTRACT=$CONTRACT"
echo ---------------------------------------------------------
echo

exit 0
