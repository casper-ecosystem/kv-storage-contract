use casper_engine_test_support::{Code, Hash, SessionBuilder, TestContext, TestContextBuilder};
use casper_types::{
    account::AccountHash,
    bytesrepr::{Bytes, FromBytes},
    runtime_args,
    CLTyped,
    RuntimeArgs,
    U512,
    PublicKey,
    SecretKey
};

pub const TEST_ACCOUNT: [u8; 32] = [7u8; 32];
pub const TEST_ADDRESS: [u8; 32] = [8u8; 32];
pub const TEST_ACCOUNT_HASH: AccountHash = AccountHash::new(TEST_ADDRESS);
pub const KV_STORAGE: &str = "kvstorage_contract";
pub const KV_STORAGE_HASH: &str = "kvstorage_contract_hash";

pub struct KVstorageContract {
    pub context: TestContext,
    pub kvstorage_hash: Hash,
}

impl KVstorageContract {
    pub fn deploy() -> Self {
        let test_account_public_key: PublicKey = SecretKey::ed25519(TEST_ACCOUNT).into();
        let mut context = TestContextBuilder::new()
            .with_public_key(test_account_public_key, TEST_ACCOUNT_HASH, U512::from(128_000_000_000_000u64))
            .build();
        let session_code = Code::from("contract.wasm");
        let session = SessionBuilder::new(session_code, runtime_args! {})
            .with_address(TEST_ACCOUNT_HASH)
            .with_authorization_keys(&[TEST_ACCOUNT_HASH])
            .build();
        context.run(session);
        let kvstorage_hash = Self::contract_hash(&context, KV_STORAGE_HASH);
        Self {
            context,
            kvstorage_hash,
        }
    }

    pub fn contract_hash(context: &TestContext, name: &str) -> Hash {
        context
            .query(TEST_ACCOUNT_HASH, &[name.to_string()])
            .unwrap_or_else(|_| panic!("{} contract not found", name))
            .into_t()
            .unwrap_or_else(|_| panic!("{} is not a type Contract.", name))
    }

    pub fn call_store_u64(&mut self, name: String, value: u64) {
        let code = Code::Hash(self.kvstorage_hash, "store_u64".to_string());
        let args = runtime_args! {
            "name" => name,
            "value" => value,
        };
        let session = SessionBuilder::new(code, args)
            .with_address(TEST_ACCOUNT_HASH)
            .with_authorization_keys(&[TEST_ACCOUNT_HASH])
            .build();
        self.context.run(session);
    }

    pub fn call_store_string(&mut self, name: String, value: String) {
        let code = Code::Hash(self.kvstorage_hash, "store_string".to_string());
        let args = runtime_args! {
            "name" => name,
            "value" => value,
        };
        let session = SessionBuilder::new(code, args)
            .with_address(TEST_ACCOUNT_HASH)
            .with_authorization_keys(&[TEST_ACCOUNT_HASH])
            .build();
        self.context.run(session);
    }

    pub fn call_store_u512(&mut self, name: String, value: U512) {
        let code = Code::Hash(self.kvstorage_hash, "store_u512".to_string());
        let args = runtime_args! {
            "name" => name,
            "value" => value,
        };
        let session = SessionBuilder::new(code, args)
            .with_address(TEST_ACCOUNT_HASH)
            .with_authorization_keys(&[TEST_ACCOUNT_HASH])
            .build();
        self.context.run(session);
    }

    pub fn call_store_bytes(&mut self, name: String, value: Bytes) {
        let code = Code::Hash(self.kvstorage_hash, "store_bytes".to_string());
        let args = runtime_args! {
            "name" => name,
            "value" => value,
        };
        let session = SessionBuilder::new(code, args)
            .with_address(TEST_ACCOUNT_HASH)
            .with_authorization_keys(&[TEST_ACCOUNT_HASH])
            .build();
        self.context.run(session);
    }

    pub fn call_store_account(&mut self, name: String, value: AccountHash) {
        let code = Code::Hash(self.kvstorage_hash, "store_account_hash".to_string());
        let args = runtime_args! {
            "name" => name,
            "value" => value,
        };
        let session = SessionBuilder::new(code, args)
            .with_address(TEST_ACCOUNT_HASH)
            .with_authorization_keys(&[TEST_ACCOUNT_HASH])
            .build();
        self.context.run(session);
    }

    pub fn query_contract<T: CLTyped + FromBytes>(&self, name: &str) -> Option<T> {
        match self
            .context
            .query(TEST_ACCOUNT_HASH, &[KV_STORAGE.to_string(), name.to_string()])
        {
            Err(_) => None,
            Ok(maybe_value) => {
                let value = maybe_value
                    .into_t()
                    .unwrap_or_else(|_| panic!("{} is not expected type.", name));
                Some(value)
            }
        }
    }
}
