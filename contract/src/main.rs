#![no_main]
#![allow(unused_imports)]
#![allow(unused_parens)]
#![allow(non_snake_case)]

extern crate alloc;
use alloc::{collections::BTreeSet, string::String};
use std::{
    collections::BTreeMap,
    convert::{TryFrom, TryInto},
};

use contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};

use types::{
    account::AccountHash,
    bytesrepr::{FromBytes, ToBytes},
    contracts::{EntryPoint, EntryPointAccess, EntryPointType, EntryPoints},
    runtime_args, ApiError, CLType, CLTyped, CLValue, Group, Key, Parameter, PublicKey,
    RuntimeArgs, URef, U128, U256, U512,
};

// Create a new storage value under dict_key in the dictionary accessed by uref
#[no_mangle]
fn create() {
    let name: String = runtime::get_named_arg("uref");
    let key: String = runtime::get_named_arg("dict_key");
    let value: Option<String> = runtime::get_named_arg("dict_value");
    create_key(name.as_str(), key.as_str(), value);
}

// Read value stored under dict_key in the dictionary accessed by uref
#[no_mangle]
fn read() {
    let name: String = runtime::get_named_arg("uref");
    let key: String = runtime::get_named_arg("dict_key");
    runtime::ret(
        CLValue::from_t(read_key::<Option<String>>(name.as_str(), key.as_str())).unwrap_or_revert(),
    );
}

// Update a value stored under dict_key in the dictionary accessed by uref with new value
#[no_mangle]
fn update() {
    let name: String = runtime::get_named_arg("uref");
    let key: String = runtime::get_named_arg("dict_key");
    let value: Option<String> = runtime::get_named_arg("dict_value");
    update_key(name.as_str(), key.as_str(), value);
}

// Delete a value stored under dict_key in the dictionary accessed by uref, Initialize with None
#[no_mangle]
fn delete() {
    let name: String = runtime::get_named_arg("uref");
    let key: String = runtime::get_named_arg("dict_key");
    update_key::<Option<String>>(name.as_str(), key.as_str(), None);
}

#[no_mangle]
pub extern "C" fn call() {
    let (contract_package_hash, _) = storage::create_contract_package_at_hash();
    let mut entry_points = EntryPoints::new();
    entry_points.add_entry_point(EntryPoint::new(
        String::from("create"),
        vec![
            Parameter::new("uref", CLType::String),
            Parameter::new("dict_key", CLType::String),
            Parameter::new("dict_value", CLType::Option(Box::new(CLType::String))),
        ],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        String::from("read"),
        vec![
            Parameter::new("uref", CLType::String),
            Parameter::new("dict_key", CLType::String),
        ],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        String::from("update"),
        vec![
            Parameter::new("uref", CLType::String),
            Parameter::new("dict_key", CLType::String),
            Parameter::new("dict_value", CLType::Option(Box::new(CLType::String))),
        ],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        String::from("delete"),
        vec![Parameter::new("uref", CLType::String)],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    let (contract_hash, _) =
        storage::add_contract_version(contract_package_hash, entry_points, Default::default());
    runtime::put_key("kvstorage_contract_hash", contract_hash.into());
    let contract_hash_pack = storage::new_uref(contract_hash);
    runtime::put_key("kvstorage_contract_hash_wrapped", contract_hash_pack.into())
}

fn create_key<T: ToBytes + CLTyped>(name: &str, dict_name: &str, dict_value: T) {
    match runtime::get_key(name) {
        None => {
            // storage::new_dictionary function creates new URef that represents a seed for a dictionary partition
            // of the global state and puts it under named keys.
            // Any value can be stored directly under named keys using storage::new_uref function.
            // But for more organisational structure, it's better to store values under dictionary_item_key in the
            // dictionary and control this dictionary with one dictionary_seed_uref.
            // It's possible to have multiple dictionaries for big data structure.
            let uref = storage::new_dictionary(dict_name).unwrap_or_revert();
            storage::dictionary_put(uref, dict_name, dict_value);
            runtime::put_key(name, uref.into());
        }
        Some(_) => {
            runtime::revert(ApiError::None);
        }
    }
}

fn read_key<T: FromBytes + CLTyped>(name: &str, dict_name: &str) -> T {
    match runtime::get_key(name) {
        Some(key) => match key {
            Key::URef(uref) => {
                let result = storage::dictionary_get::<T>(uref, dict_name).unwrap_or_revert();
                result.unwrap()
            }
            _ => runtime::revert(ApiError::None),
        },
        None => {
            runtime::revert(ApiError::None);
        }
    }
}

fn update_key<T: ToBytes + CLTyped>(name: &str, dict_name: &str, dict_value: T) {
    match runtime::get_key(name) {
        Some(key) => {
            let uref = key.try_into().unwrap_or_revert();
            storage::dictionary_put(uref, dict_name, dict_value);
            runtime::put_key(name, uref.into());
        }
        None => {
            runtime::revert(ApiError::None);
        }
    }
}
