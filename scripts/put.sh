#!/bin/bash
DEFINE_WASM=target/wasm32-unknown-unknown/release/kv_storage.wasm

KEY=$1
VALUE=$2

RESPONSE=$(casperlabs-client --host deploy.casperlabs.io deploy \
    --private-key keys/key.private.key \
    --payment-amount 10 \
    --session $DEFINE_WASM \
    --session-args '[{"name": "key", "value": {"string_value": "'$KEY'"}}, {"name": "value", "value": {"string_value": "'$VALUE'"}}]'
)

HASH=$(echo $RESPONSE | awk '{print $3}')

echo "Deployed with hash $HASH"

sleep 5

casperlabs-client --host deploy.casperlabs.io show-deploy $HASH
