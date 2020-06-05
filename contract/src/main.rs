#![cfg_attr(
    not(target_arch = "wasm32"),
    crate_type = "target arch should be wasm32"
)]
#![no_main]

use casperlabs_contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};
use casperlabs_types::{
    account::PublicKey,
    bytesrepr::{Error as BytesError, FromBytes, ToBytes},
    ApiError, CLTyped, ContractRef, Key, U512,
};
use std::convert::TryInto;

const TYPE_STRING: &str = "string";
const TYPE_U64: &str = "u64";
const TYPE_PUBLIC_KEY: &str = "public_key";
const TYPE_U512: &str = "U512";
const FN_NAME: &str = "kv_storage";

fn get_arg<T: CLTyped + FromBytes>(i: u32) -> T {
    runtime::get_arg(i).unwrap_or_revert().unwrap_or_revert()
}

fn type_name() -> String {
    let maybe_type: Result<String, BytesError> = runtime::get_arg(1).unwrap_or_revert();
    match maybe_type {
        Ok(type_of) => type_of,
        Err(_) => {
            let (type_of, _): (String, [u8; 32]) = get_arg(1);
            type_of
        }
    }
}

pub fn from_args(key: String) {
    let type_of: String = type_name();
    match type_of.as_str() {
        TYPE_STRING => {
            let value: String = get_arg(2);
            set_key(key.as_str(), value);
        }
        TYPE_U64 => {
            let value: u64 = get_arg(2);
            set_key(key.as_str(), value);
        }
        TYPE_U512 => {
            let value: U512 = get_arg(2);
            set_key(key.as_str(), value);
        }
        TYPE_PUBLIC_KEY => {
            let value: PublicKey = get_arg(2);
            set_key(key.as_str(), value);
        }
        _ => runtime::revert(ApiError::User(41)),
    }
}

pub fn key<T: FromBytes + CLTyped>(name: &str) -> Option<T> {
    match runtime::get_key(name) {
        None => None,
        Some(maybe_key) => {
            let key = maybe_key.try_into().unwrap_or_revert();
            let value = storage::read(key).unwrap_or_revert().unwrap_or_revert();
            Some(value)
        }
    }
}

pub fn set_key<T: ToBytes + CLTyped>(name: &str, value: T) {
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

#[no_mangle]
pub extern "C" fn call() {
    let contract_ref: ContractRef = storage::store_function_at_hash(FN_NAME, Default::default());
    let fn_code: Key = contract_ref.into();
    set_key(FN_NAME, fn_code);
}

#[no_mangle]
fn kv_storage() {
    let key: String = runtime::get_arg(0).unwrap().unwrap();
    from_args(key);
}
