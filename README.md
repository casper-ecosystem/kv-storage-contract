## How to understand the contract
Refer to this [guide](https://docs.casperlabs.io/en/latest/dapp-dev-guide/kv-storage-tutorial.html) to understand the Key-Value contract works, in additional to some detail on how the Casperlabs Contract DSL works. 

## Online IDE
Follow this link to execute this contract in an online IDE.
[![Open in Gitpod](https://gitpod.io/button/open-in-gitpod.svg)](https://gitpod.io/#snapshot/9fcfabb4-fe86-4452-8881-28bcbb4b5806)



## Deploying to the Testnet and Interacting with the Contract
There is a standalone python cli application that you can use for the kvstorage contract. When working with the testnet, create an account in [CLarity](https://clarity.casperlabs.io) and fund it using the faucet. Download the private key and use the key to sign the deployment. It's possible to create keys using the python client as well.

**Note, that this client was designed specifically for this contract. **


### Deploy the Contract
The first step is actually to deploy the compiled wasm to the network, if you are using the python kv-client you must use the command `deploy_kv_storage_contract`. 
Once the contract is deployed, the client will retrieve the contract session hash as well as the blockhash where the contract is deployed.

```bash
python cli.py deploy_kv_storage_contract -f "29acb007dfa4f92fa5155cc2f3ae008b4ff234acf95b00c649e2eb77447f47ca" -p "../../kvkey.private.key" -c "../target/wasm32-unknown-unknown/release/contract.wasm" -b True

```

### Invoke an Entry Point & Set a value

Once the contract is deployed, we can create another deploy, which calls one of the entry points within the contract. 
To call an entry point, you must first know the name of the entry point and the session hash, which we retrieved from the previous step. 
The kv-client, has four distinct commands to set key-values for u64, String, U512 and AccountHash.


```bash
python cli.py insert_u64 -f "29acb007dfa4f92fa5155cc2f3ae008b4ff234acf95b00c649e2eb77447f47ca" -p "../../kvkey.private.key" -s "0e82027493b88db434e85f82f6bcf48a30e0c1db15cf55fb87b73461b8aef20b" -k "test" -v 1 -b True
```


### Query the Contract On Chain
Contracts can be executed under different contexts. In this example, 
when the contract is deployed, it runs in the context of a `Contract` and not a ` Session`. 
This means that all stored keys are not stored under the account hash, but within the context of the contract. 
Therefore when we query to retrieve the value under a key, we are actually querying 
`AccountHash/kvstorage_contract/<key-name>` and not just `AccountHash/<key-name>`. 

Reading a value is simple enough, after you insert a value, the command retrieves the block hash under which the value, is stored. 
Using that block hash, and the `read-key` command you can easily retrieve and value that was previously stored under a named key.


```bash
python cli.py read_key -b "cb08a634c9bbea695fbd92e2ddbeec6fe6a374db807b36fea35077a9c1d720df" -p "29acb007dfa4f92fa5155cc2f3ae008b4ff234acf95b00c649e2eb77447f47ca" -k "test"

```

More information on the kv-client, is available via the `--help` command. There is detailed information on each of the commands available with the client. 

NOTE: The session hash is retrieved from the the chain by using a simple time delay, if processing the deploy takes longer than expected, 
it is likely that the kv-client will error out and not retrieve the session hash. 
In such cases, you can retrieve the session hash using the python casperlabs_client.

You must first find the block hash for the block that contains your deploy.
Once you have the requisite block hash, then you can use the python shell to retrieve the session hash

```bash
Import casperlabs_client
client = casperlabs_client.CasperLabsClient(‘deploy.casperlabs.io’, 40401)
Session_code = client.queryState(<block-hash>, <account-hash>, “kvstorage_contract_hash”,’address’)
Session_hash = session_code.cl_value.value.bytes_value.hex()
``` 
