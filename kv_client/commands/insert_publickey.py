from typing import Dict

from kv_storage_client import KVStorageClient

NAME: str = "insert_account_hash"
HELP: str = "Insert or update a Account Hash value with a given key name"
OPTIONS = [
	[
		("-f","--from-addr"),
		dict(
			required=True,
			type=str,
			default=None,
			help="Public Key has a base16 encoded string"
		),
	],
	[
		("-p","--private-key"),
		dict(
			required=True,
			default=None,
			type=str,
			help="Path to private associated with the account which deployed the smart contract"
		),
	],
	[
		("-s","--session-hash"),
		dict(
			required=True,
			type=str,
			default=None,
			help="Session hash of the stored contract"
		),
	],
	[
		("-k","--key"),
		dict(
			required=True,
			type=str,
			default=None,
			help="Name of the key under which a value will be saved"
		),
	],
	[
		("-v","--value-account-hash"),
		dict(
			required=True,
			type=str,
			default=None,
			help="A Base16 encoded string for a Public key value that will be saved under the given name"
		),
	],
	[
		("-b","--blocking"),
		dict(
			required=True,
			type=bool,
			default=None,
			help="Blocking Flag"
		),
	],
]

def method(client: KVStorageClient, args: Dict):
	print(args.get("value_publickey"))
	client.insert_publickey(
		from_addr=args.get("from_addr"),
		private_key=args.get("private_key"),
		session_hash=args.get("session_hash"),
		name=args.get("key"),
		key_value=args.get("value_account_hash"),
		block=args.get("blocking"),
	)