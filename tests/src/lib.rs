#[cfg(test)]
mod kv_storage;

#[cfg(test)]
mod tests {
    use super::kv_storage;
    use casper_types::{account::AccountHash, bytesrepr::Bytes, U512};
    use kv_storage::KVstorageContract;

    #[test]
    fn should_store_u64() {
        let mut kv_storage = KVstorageContract::deploy();
        let (name, value) = ("test_u64", 1u64);
        kv_storage.call_store_u64(name, value);
        let check: u64 = kv_storage.query_contract(name).unwrap();
        assert_eq!(value, check);
    }

    #[test]
    fn should_store_string() {
        let mut kv_storage = KVstorageContract::deploy();
        let (name, value) = ("test_string", String::from("Hello World"));
        kv_storage.call_store_string(name, value);
        let check: String = kv_storage.query_contract(name).unwrap();
        assert_eq!(String::from("Hello World"), check);
    }

    #[test]
    fn should_store_bytes() {
        let mut kv_storage = KVstorageContract::deploy();
        let (name, value): (&str, Bytes) = ("test_string", vec![0x41u8, 0x41u8, 0x42u8].into());
        kv_storage.call_store_bytes(name, value.clone());
        let check: Bytes = kv_storage.query_contract(name).unwrap();
        assert_eq!(value, check);
    }

    #[test]
    fn should_store_u512() {
        let mut kv_storage = KVstorageContract::deploy();
        let (name, value) = ("test_u512", U512::from(100));
        kv_storage.call_store_u512(name, value);
        let check: U512 = kv_storage.query_contract(name).unwrap();
        assert_eq!(value, check);
    }

    #[test]
    fn should_store_account_hash() {
        let mut kv_storage = KVstorageContract::deploy();
        let (name, value) = ("test_account_hash", AccountHash::new([7u8; 32]));
        kv_storage.call_store_account(name, value);
        let check: AccountHash = kv_storage.query_contract(name).unwrap();
        assert_eq!(value, check);
    }

    #[test]
    fn should_update_u64() {
        let mut kv_storage = KVstorageContract::deploy();
        let name = "test_u64";
        let original_value: u64 = 1;
        let updated_value: u64 = 2;
        kv_storage.call_store_u64(name, original_value);
        kv_storage.call_store_u64(name, updated_value);
        let value: u64 = kv_storage.query_contract(name).unwrap();
        assert_eq!(value, 2);
    }

    #[test]
    fn should_update_string() {
        let mut kv_storage = KVstorageContract::deploy();
        let name = "teststring";
        let original_value: String = String::from("Hello");
        let updated_value: String = String::from("World");
        kv_storage.call_store_string(name, original_value);
        kv_storage.call_store_string(name, updated_value);
        let value: String = kv_storage.query_contract(name).unwrap();
        assert_eq!(value, String::from("World"));
    }

    #[test]
    fn should_update_u512() {
        let mut kv_storage = KVstorageContract::deploy();
        let name = "test_u512";
        let original_value: U512 = U512::from(100);
        let updated_value: U512 = U512::from(200);
        kv_storage.call_store_u512(name, original_value);
        kv_storage.call_store_u512(name, updated_value);
        let value: U512 = kv_storage.query_contract(name).unwrap();
        assert_eq!(value, U512::from(200));
    }
    #[test]
    fn should_update_account_hash() {
        let mut kv_storage = KVstorageContract::deploy();
        let name = "test_AccountHash";
        let original_value: AccountHash = AccountHash::new([7u8; 32]);
        let updated_value: AccountHash = AccountHash::new([3u8; 32]);
        kv_storage.call_store_account(name, original_value);
        kv_storage.call_store_account(name, updated_value);
        let value: AccountHash = kv_storage.query_contract(name).unwrap();
        assert_eq!(value, updated_value);
    }
}
