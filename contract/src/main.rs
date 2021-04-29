#![no_main]
#![allow(unused_imports)]
#![allow(unused_parens)]
#![allow(non_snake_case)]

extern crate alloc;
use alloc::{collections::BTreeSet, string::String};
use std::{collections::BTreeMap, convert::TryInto};

use contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};

use types::{
    account::AccountHash,
    bytesrepr::{FromBytes, ToBytes},
    contracts::{EntryPoint, EntryPointAccess, EntryPointType, EntryPoints},
    runtime_args, CLType, CLTyped, Group, Key, Parameter, PublicKey, RuntimeArgs, URef, U128, U256,
    U512,
};

#[no_mangle]
pub extern "C" fn store_bool() {
    read_and_store::<bool>();
}

#[no_mangle]
pub extern "C" fn store_i32() {
    read_and_store::<i32>();
}

#[no_mangle]
pub extern "C" fn store_i64() {
    read_and_store::<i64>();
}

#[no_mangle]
pub extern "C" fn store_u8() {
    read_and_store::<u8>();
}

#[no_mangle]
pub extern "C" fn store_u32() {
    read_and_store::<u32>();
}

#[no_mangle]
pub extern "C" fn store_u64() {
    read_and_store::<u64>();
}

#[no_mangle]
pub extern "C" fn store_u128() {
    read_and_store::<U128>();
}

#[no_mangle]
pub extern "C" fn store_u256() {
    read_and_store::<U256>();
}

#[no_mangle]
pub extern "C" fn store_u512() {
    read_and_store::<U512>();
}

#[no_mangle]
pub extern "C" fn store_string() {
    read_and_store::<String>();
}

#[no_mangle]
pub extern "C" fn store_key() {
    read_and_store::<Key>();
}

#[no_mangle]
pub extern "C" fn store_list_of_bytes() {
    read_and_store::<Vec<u8>>();
}

#[no_mangle]
pub extern "C" fn store_public_key() {
    read_and_store::<PublicKey>();
}

#[no_mangle]
pub extern "C" fn store_option() {
    read_and_store::<Option<String>>();
}

#[no_mangle]
pub extern "C" fn store_result() {
    read_and_store::<Result<String, String>>();
}

#[no_mangle]
pub extern "C" fn store_byte_array() {
    read_and_store::<[u8; 3]>();
}

#[no_mangle]
pub extern "C" fn store_map() {
    read_and_store::<BTreeMap<String, Option<String>>>();
}

#[no_mangle]
pub extern "C" fn store_tuple3() {
    read_and_store::<(PublicKey, Option<String>, U512)>();
}

#[no_mangle]
pub extern "C" fn store_tuple2() {
    read_and_store::<(String, U512)>();
}

#[no_mangle]
pub extern "C" fn call() {
    let (contract_package_hash, _) = storage::create_contract_package_at_hash();
    let mut entry_points = EntryPoints::new();

    entry_points.add_entry_point(endpoint("store_bool", CLType::Bool));
    entry_points.add_entry_point(endpoint("store_i32", CLType::I32));
    entry_points.add_entry_point(endpoint("store_i64", CLType::I64));
    entry_points.add_entry_point(endpoint("store_u8", CLType::U8));
    entry_points.add_entry_point(endpoint("store_u32", CLType::U32));
    entry_points.add_entry_point(endpoint("store_u64", CLType::U64));
    entry_points.add_entry_point(endpoint("store_u128", CLType::U128));
    entry_points.add_entry_point(endpoint("store_u256", CLType::U256));
    entry_points.add_entry_point(endpoint("store_u512", CLType::U512));
    entry_points.add_entry_point(endpoint("store_string", CLType::String));
    entry_points.add_entry_point(endpoint("store_key", CLType::Key));
    entry_points.add_entry_point(endpoint(
        "store_list_of_bytes",
        CLType::List(Box::new(CLType::U8)),
    ));
    entry_points.add_entry_point(endpoint("store_public_key", CLType::PublicKey));
    entry_points.add_entry_point(endpoint(
        "store_option",
        CLType::Option(Box::new(CLType::String)),
    ));
    entry_points.add_entry_point(endpoint(
        "store_result",
        CLType::Result {
            ok: Box::new(CLType::String),
            err: Box::new(CLType::String),
        },
    ));
    entry_points.add_entry_point(endpoint("store_byte_array", CLType::ByteArray(3)));
    entry_points.add_entry_point(endpoint(
        "store_map",
        CLType::Map {
            key: Box::new(CLType::String),
            value: Box::new(CLType::Option(Box::new(CLType::String))),
        },
    ));
    entry_points.add_entry_point(endpoint(
        "store_tuple3",
        CLType::Tuple3([
            Box::new(CLType::PublicKey),
            Box::new(CLType::Option(Box::new(CLType::String))),
            Box::new(CLType::U512),
        ]),
    ));
    entry_points.add_entry_point(endpoint(
        "store_tuple2",
        CLType::Tuple2([Box::new(CLType::String), Box::new(CLType::U512)]),
    ));

    let (contract_hash, _) =
        storage::add_contract_version(contract_package_hash, entry_points, Default::default());
    runtime::put_key("kvstorage_contract", contract_hash.into());
    let contract_hash_pack = storage::new_uref(contract_hash);
    runtime::put_key("kvstorage_contract_hash", contract_hash_pack.into())
}

fn read_and_store<T: CLTyped + FromBytes + ToBytes>() {
    let name: String = runtime::get_named_arg("name");
    let value: T = runtime::get_named_arg("value");
    set_key(name.as_str(), value);
}

fn endpoint(name: &str, value_type: CLType) -> EntryPoint {
    EntryPoint::new(
        String::from(name),
        vec![
            Parameter::new("name", CLType::String),
            Parameter::new("value", value_type),
        ],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )
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
