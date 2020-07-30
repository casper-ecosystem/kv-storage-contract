extern crate alloc;
use alloc::{collections::BTreeSet, string::String};
use std::convert::TryInto;

use contract_macro::{casperlabs_constructor, casperlabs_contract, casperlabs_method};

use contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};

use types::{
    account::AccountHash,
    bytesrepr::{FromBytes, ToBytes},
    contracts::{EntryPoint, EntryPointAccess, EntryPointType, EntryPoints},
    runtime_args, CLType, CLTyped, CLValue, Group, Parameter, RuntimeArgs, URef, U512,
};

#[casperlabs_contract]
mod kvstorage_contract {

    #[casperlabs_constructor]
    fn init() {}

    #[casperlabs_method]
    fn store_u64(name: String, value: u64) {
        println!("{}", value);
        set_key(name.as_str(), value);
    }

    #[casperlabs_method]
    fn get_u64(name: String) -> u64 {
        key(name.as_str())
    }

    #[casperlabs_method]
    fn get_string(name: String) -> String {
        key(name.as_str())
    }

    #[casperlabs_method]
    fn store_u512(name: String, value: U512) {
        set_key(name.as_str(), value);
    }

    #[casperlabs_method]
    fn store_string(name: String, value: String) {
        set_key(name.as_str(), value);
    }

    #[casperlabs_method]
    fn store_account_hash(name: String, value: AccountHash) {
        set_key(name.as_str(), value);
    }

    fn key<T: FromBytes + CLTyped>(name: &str) -> T {
        let key = runtime::get_key(name)
            .unwrap_or_revert()
            .try_into()
            .unwrap_or_revert();
        storage::read(key).unwrap_or_revert().unwrap_or_revert()
    }

    fn set_key<T: ToBytes + CLTyped>(name: &str, value: T) {
        match runtime::get_key(name) {
            Some(key) => {
                let key_ref = key.try_into().unwrap_or_revert();
                storage::write(key_ref, value);
            }
            None => {
                let key = storage::new_uref(value).into();
                runtime::put_key(name, key);
            }
        }
    }
}
