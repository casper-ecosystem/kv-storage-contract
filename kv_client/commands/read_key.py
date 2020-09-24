from typing import Dict

from kv_storage_client import KVStorageClient

NAME: str = "read_key"
HELP: str = ("Return a CLType object stored under the given named key")

OPTIONS = [
	[
		("-b","--block-hash"),
		dict(
			required=True,
			type=str,
			default=None,
			help="Base16 encoded hash of block under which the query takes place"
		),
	],
	[
		("-a","--account-hash"),
		dict(
			required=True,
			type=str,
			default=None,
			help="Base16 encoded public_key key of the account",
		),
	],
	[
		("-k","--key"),
		dict(
			required=True,
			type=str,
			default=None,
			help="Name of the key",
		),
	],
]

def method(client: KVStorageClient, args: Dict):
	client.read_key(
		block_hash=args.get("block_hash"),
		public_key=args.get("account_hash"),
		key=args.get("key"),
	)