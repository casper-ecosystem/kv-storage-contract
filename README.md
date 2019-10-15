# Key Value Storage

This is an example of a simple string-base key-value smart contract and it's usage.

# Usage

## Step 1 - Compile smart contracts.
```bash
$ cargo build --release
```

## Step 2 - Save value under a key.
Make sure to run scripts in the root directory!
```bash
$ ./scripts/put.sh "answer" "43"
```

## Step 3 - Check the value.
```bash
$ ./scripts/get.sh "answer"
```
Value of the counter should be `43`.

## Step 4 - Update the value.
```bash
$ ./scripts/put.sh "answer" "42"
```

## Step 5 - Check the value again.
```bash
$ ./scripts/get.sh "answer"
```
Value of the counter should be `42`.

## GraphQL
You can check the value of the counter using devnet's GraphQL console:
https://devnet-graphql.casperlabs.io

Go to and then:

### Check latest block hash in 
```
query {
  dagSlice(depth: 1) {
      blockHash
  }
}
```

### Get public key of your account
```bash
$ cat keys/key.public.hex.key
```

### Check the counter value.
Put block hash under `blockHashBase16Prefix` and your public key under `keyBase`. Put your key in `pathSegments` like in the example.
```
query {
  globalState(
    blockHashBase16Prefix: "96720f16a215b5e55f1a7475256370f48efa932248b7bcd633d29413a5c1f033"
    StateQueries: [
      {
        keyType: Address
        keyBase16: "64d0c86f888e925731cae4398c6ea86d26a14e2574e70b36bd4eeaec3a292cde"
        pathSegments: ["answer"]
      }
    ]
  ) {
    value {
      __typename
      ... on IntValue {
        int: value
      }
    }
  }
}
```