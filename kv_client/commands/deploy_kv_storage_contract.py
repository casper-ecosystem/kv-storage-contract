from typing import Dict

from kv_storage_client import KVStorageClient

NAME: str = "deploy_kv_storage_contract"
HELP: str = (
	"Deploy the Key Value storage contract under the named key kv_storage"
)
OPTIONS = [
	[
		("-f","--from-addr"),
		dict(
			required=True,
			type=str,
			default=None,
			help="Public Key address as a base16 encoded string"),
	],
	[
		("-p","--private-key"),
		dict(
			required=True,
			type=str,
			help="Path to the file with account private key"),
	],
	[
		("-b","--blocking"),
		dict(
			required=False,
			type=bool,
			default=True,
			help="blocking"),
	],

]

def method(client: KVStorageClient, args: Dict):
	print(args)
	client.deploy_kv_storage_contract(
		from_addr=args.get("from_addr"),
		private_key=args.get("private_key"),
		block=args.get("blocking"),
	)