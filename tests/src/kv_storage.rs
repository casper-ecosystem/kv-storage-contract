use casper_engine_test_support::{Code, Hash, SessionBuilder, TestContext, TestContextBuilder};
use casper_types::{
    account::AccountHash,
    bytesrepr::{FromBytes, ToBytes},
    runtime_args, AsymmetricType, CLTyped, PublicKey, RuntimeArgs, U512,
};

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
        let account_hash = account.to_account_hash();
        let mut context = TestContextBuilder::new()
            .with_public_key(account, U512::from(500_000_000_000_000_000u64))
            .build();
        let session_code = Code::from("contract.wasm");
        let session = SessionBuilder::new(session_code, runtime_args! {})
            .with_address(account_hash)
            .with_authorization_keys(&[account_hash])
            .build();
        context.run(session);
        let kvstorage_hash = context
            .query(account_hash, &[KV_STORAGE_HASH.to_string()])
            .unwrap()
            .into_t()
            .unwrap();

        Self {
            context,
            kvstorage_hash,
            account: account_hash,
        }
    }

    pub fn call_store_value<T: CLTyped + ToBytes>(
        &mut self,
        fn_name: &str,
        key_name: &str,
        value: T,
    ) {
        self.call(
            fn_name,
            runtime_args! {
                "name" => key_name,
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
