#!/bin/bash

KEY=$1
PUBLIC_KEY=$(cat keys/key.public.hex.key)
RESPONSE=$(casperlabs-client --host deploy.casperlabs.io show-blocks)
BLOCK_HASH=$(echo $RESPONSE | awk -F "block_hash: \"" '{print $2}' | awk -F "\" header" '{print $1}')

echo "Value of the '$KEY' key under $PUBLIC_KEY account is:"
casperlabs-client --host deploy.casperlabs.io query-state \
    --block-hash $BLOCK_HASH \
    --type address \
    --key $PUBLIC_KEY \
    --path ""$KEY

