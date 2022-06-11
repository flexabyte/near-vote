#!/usr/bin/env bash

# exit on first error after this point to avoid redeploying with successful build
set -e
echo
echo ---------------------------------------------------------
echo "Step 0: Check for environment variable with contract name"
echo ---------------------------------------------------------
echo

[ -z "$CONTRACT" ] && echo "Missing \$CONTRACT environment variable" && exit 1
[ -z "$ACCOUNT" ] && echo "Missing \$ACCOUNT environment variable" && exit 1
[ -z "$CONTRACT" ] || [ -z "$ACCOUNT" ] || echo "\$CONTRACT is set to [ $CONTRACT ]. \$ACCOUNT is set to [ $ACCOUNT ]"

echo
echo
echo ---------------------------------------------------------
echo "Step 1: Call initialize function on the contract"
echo ---------------------------------------------------------
echo

echo "Setting voting end to 10 seconds from now."
timestamp=$(date +%s%N)
let vote_end=timestamp+10000000000
near call $CONTRACT initialize "{\"allowedOptions\": [\"Beyond\", \"Impossible\", \"Frys\", \"Squeaky Bean\"], \"endTimestamp\": $vote_end}" --accountId $ACCOUNT

echo
echo ---------------------------------------------------------
echo "Step 2: Call voting functions on the contract"
echo ---------------------------------------------------------
echo

# the following line fails with an error because we can't write to storage without signing the message
# --> FunctionCallError(HostError(ProhibitedInView { method_name: "storage_write" }))
# near view $CONTRACT write '{"key": "some-key", "value":"some value"}'
echo "Getting all voting options"
near view $CONTRACT get_options '{"option":"Beyond"}' --accountId $ACCOUNT
echo "Voting for 'Beyond'"
near call $CONTRACT add_vote '{"vote":"Beyond"}' --accountId $ACCOUNT
echo "Getting what I voted for... it should be 'Beyond'"
near call $CONTRACT get_vote '{}' --accountId $ACCOUNT
echo "Sleeping for 10 seconds... just to ensure the vote has ended before we can get results."
sleep 10
echo "Getting total votes for 'Beyond'"
near view $CONTRACT get_total_votes '{"option":"Beyond"}' --accountId $ACCOUNT

echo
echo "now run this script again to see changes made by this file"
exit 0
