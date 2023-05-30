use casper_engine_test_support::{PRODUCTION_RUN_GENESIS_REQUEST, InMemoryWasmTestBuilder, DEFAULT_ACCOUNT_ADDR, ExecuteRequestBuilder};
use casper_types::{
    account::AccountHash,
    bytesrepr::{FromBytes, ToBytes},
    runtime_args, CLTyped, RuntimeArgs, ContractHash, Key,
};

pub const KV_STORAGE: &str = "kvstorage_contract_hash";

pub struct KVstorageContract {
    pub context: InMemoryWasmTestBuilder,
    pub kvstorage_hash: ContractHash,
    pub account: AccountHash,
}

impl KVstorageContract {
    pub fn deploy() -> Self {
        let mut builder = InMemoryWasmTestBuilder::default();
        builder.run_genesis(&PRODUCTION_RUN_GENESIS_REQUEST);

        let install_request_1 = ExecuteRequestBuilder::standard(
            *DEFAULT_ACCOUNT_ADDR,
            "contract.wasm",
            RuntimeArgs::default(),
        )
        .build();
    
        builder.exec(install_request_1).expect_success().commit();
    
        let account = builder
            .get_account(*DEFAULT_ACCOUNT_ADDR)
            .expect("should have account");
    
        let kvstorage_hash = account
            .named_keys()
            .get(KV_STORAGE)
            .and_then(|key| key.into_hash())
            .map(ContractHash::new)
            .expect("should have contract hash");

        Self {
            context: builder,
            kvstorage_hash,
            account: *DEFAULT_ACCOUNT_ADDR,
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
        let request = ExecuteRequestBuilder::contract_call_by_hash(
            self.account,
            self.kvstorage_hash,
            method,
            args,
        )
        .build();
    
        self.context.exec(request).expect_success().commit();
    }

    pub fn query_contract<T: CLTyped + FromBytes>(&self, name: &str) -> Option<T> {
        self
            .context
            .query(None, Key::Account(self.account), &[KV_STORAGE.to_string(), name.to_string()])
            .expect("should be StoredValue")
            .as_cl_value()
            .cloned()
            .expect("must have cl value")
            .into_t::<T>()
            .ok()
    }
}
