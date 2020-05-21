use casperlabs_contract::args_parser::ArgsParser;
use casperlabs_engine_test_support::{Code, Hash, SessionBuilder, TestContext, TestContextBuilder};
use casperlabs_types::{account::PublicKey, bytesrepr::FromBytes, CLTyped, Key, U512};

pub const TEST_ACCOUNT: PublicKey = PublicKey::ed25519_from([7u8; 32]);
pub const KV_STORAGE: &str = "kv_storage";

pub struct KVstorageContract {
    pub context: TestContext,
    pub kvstorage_hash: Hash,
}

impl KVstorageContract {
    pub fn deploy() -> Self {
        let mut context = TestContextBuilder::new()
            .with_account(TEST_ACCOUNT, U512::from(128_000_000))
            .build();
        let session_code = Code::from("contract.wasm");
        let session = SessionBuilder::new(session_code, ())
            .with_address(TEST_ACCOUNT)
            .with_authorization_keys(&[TEST_ACCOUNT])
            .build();
        context.run(session);
        let kvstorage_hash = Self::contract_hash(&context, KV_STORAGE);
        Self {
            context,
            kvstorage_hash,
        }
    }

    pub fn contract_hash(context: &TestContext, name: &str) -> Hash {
        let contract_ref: Key = context
            .query(TEST_ACCOUNT, &[name])
            .unwrap_or_else(|_| panic!("{} contract not found", name))
            .into_t()
            .unwrap_or_else(|_| panic!("{} is not a type Contract.", name));
        contract_ref
            .into_hash()
            .unwrap_or_else(|| panic!("{} is not a type Hash", name))
    }

    pub fn call_indirect(&mut self, args: impl ArgsParser) {
        let code = Code::Hash(self.kvstorage_hash);
        let session = SessionBuilder::new(code, args)
            .with_address(TEST_ACCOUNT)
            .with_authorization_keys(&[TEST_ACCOUNT])
            .build();
        self.context.run(session);
    }

    pub fn query_contract<T: CLTyped + FromBytes>(&self, name: &str) -> Option<T> {
        match self.context.query(TEST_ACCOUNT, &[name]) {
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
