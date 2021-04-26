use casper_engine_test_support::{Code, Hash, SessionBuilder, TestContext, TestContextBuilder};
use casper_types::{
    account::AccountHash,
    bytesrepr::{Bytes, FromBytes},
    runtime_args, AsymmetricType, CLTyped, PublicKey, RuntimeArgs, U512,
};
use std::collections::BTreeMap;

pub const KV_STORAGE: &str = "kvstorage_contract";
pub const KV_STORAGE_HASH: &str = "kvstorage_contract_hash";

pub struct KVstorageContract {
    pub context: TestContext,
    pub kvstorage_hash: Hash,
    pub account: AccountHash,
}

impl KVstorageContract {
    pub fn deploy() -> Self {
        let account = PublicKey::ed25519_from_bytes([3u8; 32]).unwrap();
        let mut context = TestContextBuilder::new()
            .with_public_key(account, U512::from(500_000_000_000_000_000u64))
            .build();
        let session_code = Code::from("contract.wasm");
        let session = SessionBuilder::new(session_code, runtime_args! {})
            .with_address(account.to_account_hash())
            .with_authorization_keys(&[account.to_account_hash()])
            .build();
        context.run(session);
        let kvstorage_hash = context
            .query(account.to_account_hash(), &[KV_STORAGE_HASH.to_string()])
            .unwrap()
            .into_t()
            .unwrap();

        Self {
            context,
            kvstorage_hash,
            account: account.to_account_hash(),
        }
    }

    pub fn call_store_u64(&mut self, name: &str, value: u64) {
        self.call(
            "store_u64",
            runtime_args! {
                "name" => name,
                "value" => value
            },
        );
    }

    pub fn call_store_string(&mut self, name: &str, value: String) {
        self.call(
            "store_string",
            runtime_args! {
                "name" => name,
                "value" => value
            },
        );
    }

    pub fn call_store_u512(&mut self, name: &str, value: U512) {
        self.call(
            "store_u512",
            runtime_args! {
                "name" => name,
                "value" => value
            },
        );
    }

    pub fn call_store_bytes(&mut self, name: &str, value: Bytes) {
        self.call(
            "store_bytes",
            runtime_args! {
                "name" => name,
                "value" => value
            },
        );
    }

    pub fn call_store_account(&mut self, name: &str, value: AccountHash) {
        self.call(
            "store_account_hash",
            runtime_args! {
                "name" => name,
                "value" => value
            },
        );
    }

    pub fn call_store_public_key(&mut self, name: &str, value: PublicKey) {
        self.call(
            "store_public_key",
            runtime_args! {
                "name" => name,
                "value" => value
            },
        );
    }

    pub fn call_store_option(&mut self, name: &str, value: Option<String>) {
        self.call(
            "store_option",
            runtime_args! {
                "name" => name,
                "value" => value
            },
        );
    }

    pub fn call_store_result(&mut self, name: &str, value: Result<String, String>) {
        self.call(
            "store_result",
            runtime_args! {
                "name" => name,
                "value" => value
            },
        );
    }

    pub fn call_store_byte_array(&mut self, name: &str, value: [u8; 3]) {
        self.call(
            "store_byte_array",
            runtime_args! {
                "name" => name,
                "value" => value
            },
        );
    }

    pub fn call_store_map(&mut self, name: &str, value: BTreeMap<String, Option<String>>) {
        self.call(
            "store_map",
            runtime_args! {
                "name" => name,
                "value" => value
            },
        );
    }

    pub fn call_store_tuple(&mut self, name: &str, value: (PublicKey, Option<String>, U512)) {
        self.call(
            "store_tuple",
            runtime_args! {
                "name" => name,
                "value" => value
            },
        );
    }

    fn call(&mut self, method: &str, args: RuntimeArgs) {
        let code = Code::Hash(self.kvstorage_hash, method.to_string());
        let session = SessionBuilder::new(code, args)
            .with_address(self.account)
            .with_authorization_keys(&[self.account])
            .build();
        self.context.run(session);
    }

    pub fn query_contract<T: CLTyped + FromBytes>(&self, name: &str) -> Option<T> {
        match self
            .context
            .query(self.account, &[KV_STORAGE.to_string(), name.to_string()])
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
