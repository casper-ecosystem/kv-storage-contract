#![no_main]
#![allow(unused_imports)]
#![allow(unused_parens)]
#![allow(non_snake_case)]

extern crate alloc;
use alloc::{collections::BTreeSet, string::String};
use std::convert::TryInto;

use contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};

use types::{
    account::AccountHash,
    bytesrepr::ToBytes,
    contracts::{EntryPoint, EntryPointAccess, EntryPointType, EntryPoints},
    runtime_args, CLType, CLTyped, Group, Parameter, RuntimeArgs, URef, U512,
};

#[no_mangle]
pub extern "C" fn store_u64() {
    let name: String = runtime::get_named_arg("name");
    let value: u64 = runtime::get_named_arg("value");
    set_key(name.as_str(), value);
}

#[no_mangle]
pub extern "C" fn store_u512() {
    let name: String = runtime::get_named_arg("name");
    let value: U512 = runtime::get_named_arg("value");
    set_key(name.as_str(), value);
}

#[no_mangle]
pub extern "C" fn store_string() {
    let name: String = runtime::get_named_arg("name");
    let value: String = runtime::get_named_arg("value");
    set_key(name.as_str(), value);
}

#[no_mangle]
pub extern "C" fn store_account_hash() {
    let name: String = runtime::get_named_arg("name");
    let value: AccountHash = runtime::get_named_arg("value");
    set_key(name.as_str(), value);
}

#[no_mangle]
pub extern "C" fn store_bytes() {
    let name: String = runtime::get_named_arg("name");
    let value: Vec<u8> = runtime::get_named_arg("value");
    set_key(name.as_str(), value);
}

#[no_mangle]
pub extern "C" fn call() {
    let (contract_package_hash, _) = storage::create_contract_package_at_hash();
    let mut entry_points = EntryPoints::new();

    entry_points.add_entry_point(EntryPoint::new(
        String::from("store_u64"),
        vec![
            Parameter::new("name", CLType::String),
            Parameter::new("value", CLType::U64),
        ],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        String::from("store_u512"),
        vec![
            Parameter::new("name", CLType::String),
            Parameter::new("value", CLType::U512),
        ],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        String::from("store_string"),
        vec![
            Parameter::new("name", CLType::String),
            Parameter::new("value", CLType::String),
        ],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        String::from("store_account_hash"),
        vec![
            Parameter::new("name", CLType::String),
            Parameter::new("value", AccountHash::cl_type()),
        ],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        String::from("store_bytes"),
        vec![
            Parameter::new("name", CLType::String),
            Parameter::new("value", CLType::List(Box::new(CLType::U8))),
        ],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    let (contract_hash, _) =
        storage::add_contract_version(contract_package_hash, entry_points, Default::default());
    runtime::put_key("kvstorage_contract", contract_hash.into());
    let contract_hash_pack = storage::new_uref(contract_hash);
    runtime::put_key("kvstorage_contract_hash", contract_hash_pack.into())
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
