import casperlabs_client
from casperlabs_client import abi
import time
import argparse


class KVStorageClient:
	def __init__(self,addr,port):
		self.client = casperlabs_client.CasperLabsClient(addr,port)
		self.deploy_hash = None
		self.block_hash = None
		self.session_hash = None

	def deploy_kv_storage_contract(self, from_addr, private_key, contract_wasm_location, block):
		deploy_hash = self.client.deploy(
							from_addr = from_addr,
							private_key = private_key,
							gas_price=10,
							payment_amount=2000000,
							session=contract_wasm_location)
		print("Deploy complete. Waiting for deploy to be processed")
		if block:
			print("Start: %s" % time.ctime())
			time.sleep(5)
			print("End: %s" % time.ctime())
			block_hash = self.client.wait_for_deploy_processed(deploy_hash).processing_results[0].block_info.summary.block_hash.hex()
			self.session_code_hash(block_hash, from_addr)


	def session_code_hash(self, block_hash, public_key):
		print("Start %s" % time.ctime())
		time.sleep(5)
		print("End %s" % time.ctime())
		maybe_code = self.client.queryState(block_hash,public_key,"kv_storage",'address')
		if not maybe_code:
			self.deploy(True)
			self.session_code_hash(public_key)
		self.session_hash = maybe_code.cl_value.value.key.hash.hash
		print("Current Session Hash: [%s]" % self.session_hash.hex()) 
		print("Current Block Hash: [%s]" % block_hash)

	def read_key(self, block_hash, public_key, key):
		print(self.client.queryState(block_hash, public_key, key,'address'))


	def insert_value_with_type(self, from_addr, private_key, session_hash, name,value_type,last_arg, block):
		args = [abi.ABI.string_value("name",name),abi.ABI.string_value("type",value_type),last_arg]	
		deploy_hash = self.client.deploy(from_addr = from_addr ,
							private_key=private_key,
							gas_price=10,
							payment_amount=2000000,
							session_hash=bytes.fromhex(session_hash),
							session_args=abi.ABI.args(args))
		if block:
			print("Insert complete. Waiting for deploy to be processed.")
			time.sleep(5)
			self.block_hash = self.client.wait_for_deploy_processed(deploy_hash).processing_results[0].block_info.summary.block_hash.hex()
			print("Lastest block hash for most recent query: [%s]" % self.block_hash)

	def insert_u64(self, from_addr ,private_key, session_hash, name, u64_value, block):
		print(private_key, session_hash, name, u64_value,block)
		last_arg = abi.ABI.long_value("value",u64_value)
		self.insert_value_with_type(from_addr, private_key, session_hash, name, "u64", last_arg, block)

	def insert_string(self, from_addr, private_key, session_hash, name, string_value, block):
		last_arg = abi.ABI.string_value("value",string_value)
		self.insert_value_with_type(from_addr, private_key, session_hash, name, "string",last_arg, block)

	def insert_u512(self, from_addr, private_key, session_hash, name, u512_value, block):
		print(u512_value)
		last_arg = abi.ABI.big_int("value", u512_value)
		self.insert_value_with_type(from_addr, private_key, session_hash, name, "U512" ,last_arg, block)

	def insert_publickey(self, from_addr, private_key, session_hash, name, key_value, block):
		last_arg = abi.ABI.account("value",key_value)
		self.insert_value_with_type(from_addr, private_key, session_hash, name, "public_key" ,last_arg, block)




