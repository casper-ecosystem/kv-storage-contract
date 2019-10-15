#![no_std]

extern crate alloc;
extern crate contract_ffi;
use alloc::string::String;

pub extern "C" fn call() {
    let key: String = contract_api::get_arg(0).unwrap().unwrap();
    let value: String = contract_api::get_arg(1).unwrap().unwrap();
    let value_uref = contract_api::new_turef(value);
    contract_api::put_key(&key, &value_uref.into());
}
