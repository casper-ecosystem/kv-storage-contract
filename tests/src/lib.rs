#[cfg(test)]
mod kv_storage;

#[cfg(test)]
mod tests {
    use super::kv_storage;
    use casper_types::{account::AccountHash, bytesrepr::Bytes, AsymmetricType, PublicKey, U512};
    use kv_storage::KVstorageContract;
    use std::collections::BTreeMap;

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
    #[test]
    fn should_update_public_key() {
        let mut kv_storage = KVstorageContract::deploy();
        let name = "test_PublicKey";
        let original_value = PublicKey::ed25519_from_bytes([1u8; 32]).unwrap();
        let updated_value = PublicKey::ed25519_from_bytes([3u8; 32]).unwrap();
        kv_storage.call_store_public_key(name, original_value);
        kv_storage.call_store_public_key(name, updated_value);
        let value: PublicKey = kv_storage.query_contract(name).unwrap();
        assert_eq!(value, updated_value);
    }
    #[test]
    fn should_update_option() {
        let mut kv_storage = KVstorageContract::deploy();
        let name = "test_Option";
        let original_value = Some(String::from("Hello"));
        let updated_value = Some(String::from("World"));
        kv_storage.call_store_option(name, original_value);
        kv_storage.call_store_option(name, updated_value);
        let value: Option<String> = kv_storage.query_contract(name).unwrap();
        assert_eq!(value.unwrap(), String::from("World"));
    }
    #[test]
    fn should_update_result() {
        let mut kv_storage = KVstorageContract::deploy();
        let name = "test_Result";
        let original_value = Ok(String::from("Success"));
        let updated_value = Err(String::from("Fail"));
        kv_storage.call_store_result(name, original_value);
        kv_storage.call_store_result(name, updated_value);
        let value: Result<String, String> = kv_storage.query_contract(name).unwrap();
        assert_eq!(value.is_err(), true);
    }
    #[test]
    fn should_update_byte_array() {
        let mut kv_storage = KVstorageContract::deploy();
        let name = "test_ByteArray";
        let original_value: [u8; 3] = [1, 2, 3];
        let updated_value: [u8; 3] = [2, 4, 6];
        kv_storage.call_store_byte_array(name, original_value);
        kv_storage.call_store_byte_array(name, updated_value);
        let value: [u8; 3] = kv_storage.query_contract(name).unwrap();
        assert_eq!(value, updated_value);
    }
    #[test]
    fn should_update_map() {
        let mut kv_storage = KVstorageContract::deploy();
        let name = "test_Map";
        let mut original_value = BTreeMap::new();
        original_value.insert(String::from("alice"), Some(String::from("Purchased Milk")));
        original_value.insert(String::from("bob"), Some(String::from("Purchased Egg")));
        let mut updated_value = BTreeMap::new();
        updated_value.insert(String::from("alice"), Some(String::from("Sold Milk")));
        updated_value.insert(String::from("jane"), Some(String::from("Purchased Cake")));
        kv_storage.call_store_map(name, original_value);
        kv_storage.call_store_map(name, updated_value);
        let res: BTreeMap<String, Option<String>> = kv_storage.query_contract(name).unwrap();
        let keys: Vec<String> = res.keys().cloned().collect();
        let values: Vec<Option<String>> = res.values().cloned().collect();
        assert_eq!(keys, [String::from("alice"), String::from("jane")]);
        assert_eq!(values.get(1).unwrap(), &Some(String::from("Purchased Cake")));
    }
    #[test]
    fn should_update_tuple() {
        let mut kv_storage = KVstorageContract::deploy();
        let name = "test_Tuple";
        let original_value = (
            PublicKey::ed25519_from_bytes([1u8; 32]).unwrap(),
            Some(String::from("Original")),
            U512::from(100)
        );
        let updated_value = (
            PublicKey::ed25519_from_bytes([3u8; 32]).unwrap(),
            Some(String::from("Updated")),
            U512::from(300)
        );
        kv_storage.call_store_tuple(name, original_value);
        kv_storage.call_store_tuple(name, updated_value);
        let res: (PublicKey, Option<String>, U512) = kv_storage.query_contract(name).unwrap();
        assert_eq!(res.0, PublicKey::ed25519_from_bytes([3u8; 32]).unwrap());
        assert_eq!(res.1.unwrap(), String::from("Updated"));
        assert_eq!(res.2, U512::from(300));
    }
}
