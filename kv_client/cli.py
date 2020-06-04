import argparse
import sys
from gooey import Gooey, GooeyParser

from kv_storage_client import KVStorageClient
from commands import (
	deploy_kv_storage_contract,
	insert_u64,
	insert_string,
	insert_u512,
	insert_publickey,
	read_key
)

@Gooey(program_name='CasperLabs Key-Value Storage Client')		
def cli(*arguments) -> int:
	class Parser:
		def __init__(self):
			self.parser = GooeyParser(
				prog="kv_storage_client", add_help=False
			)
			self.parser.add_argument(
				"-ignore-gooey",
				required=False
			)
			self.parser.add_argument(
				"--help",
				action="help",
				default=argparse.SUPPRESS,
				help="show this help message and exit"
			)
			self.parser.add_argument(
				"-h",
				"--host",
				required=False,
				default="deploy.casperlabs.io",
				type=str,
				help="Hostname or IP of the node"
			)
			self.parser.add_argument(
				"-p",
				"--port",
				required=False,
				default=40401,
				type=int,
				help="Port of the node"
			)
			self.sp = self.parser.add_subparsers(help="Choose a request")

			def no_command(kv_storage_client, args):
				print("Please provide a method. --help for usage")
				self.parser.print_usage()
				return 1

			self.parser.set_defaults(function=no_command)

		def add_command(self, command_name: str, function, help_message: str, argument_list: list):
			command_parser = self.sp.add_parser(command_name, help=help_message)
			command_parser.set_defaults(function=function)
			for (args,options) in argument_list:
				command_parser.add_argument(*args, **options)

		def run(self, argv):
			print(self.parser.parse_args(argv))
			args = vars(self.parser.parse_args(argv))
			return args["function"](
				KVStorageClient(
					args.get("host"),
					args.get("port")
				),
				args,
			)

	parser = Parser()

	for command in (
		deploy_kv_storage_contract,
		insert_u64,
		insert_string,
		insert_u512,
		insert_publickey,
		read_key
	):
		parser.add_command(command.NAME, command.method, command.HELP, command.OPTIONS)

	return parser.run([str(a) for a in arguments])

def main():
	return cli(*sys.argv[1:])

if __name__ == "__main__":
	sys.exit(main())