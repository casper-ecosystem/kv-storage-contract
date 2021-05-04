#[cfg(test)]
mod kv_storage;

#[cfg(test)]
mod tests {
    use super::kv_storage;
    use casper_types::{
        account::AccountHash,
        bytesrepr::{Bytes, FromBytes, ToBytes},
        AsymmetricType, CLTyped, Key, PublicKey, U128, U256, U512,
    };
    use kv_storage::KVstorageContract;
    use std::collections::BTreeMap;

    fn generic_test<T: CLTyped + FromBytes + ToBytes>(
        fn_name: &str,
        key_name: &str,
        value1: T,
        value2: T,
    ) -> (T, T) {
        let mut kv_storage = KVstorageContract::deploy();
        kv_storage.call_store_value(fn_name, key_name, value1);
        let check1: T = kv_storage.query_contract(key_name).unwrap();
        kv_storage.call_store_value(fn_name, key_name, value2);
        let check2: T = kv_storage.query_contract(key_name).unwrap();
        (check1, check2)
    }

    #[test]
    fn should_store_bool() {
        let (value1, value2): (bool, bool) = (true, false);
        let (ret1, ret2) = generic_test::<bool>("store_bool", "test_bool", value1, value2);
        assert_eq!(value1, ret1);
        assert_eq!(value2, ret2);
    }

    #[test]
    fn should_store_i32() {
        let (value1, value2): (i32, i32) = (7i32, 9i32);
        let (ret1, ret2) = generic_test::<i32>("store_i32", "test_i32", value1, value2);
        assert_eq!(value1, ret1);
        assert_eq!(value2, ret2);
    }

    #[test]
    fn should_store_i64() {
        let (value1, value2): (i64, i64) = (3i64, 5i64);
        let (ret1, ret2) = generic_test::<i64>("store_i64", "test_i64", value1, value2);
        assert_eq!(value1, ret1);
        assert_eq!(value2, ret2);
    }

    #[test]
    fn should_store_u8() {
        let (value1, value2): (u8, u8) = (1u8, 3u8);
        let (ret1, ret2) = generic_test::<u8>("store_u8", "test_u8", value1, value2);
        assert_eq!(value1, ret1);
        assert_eq!(value2, ret2);
    }

    #[test]
    fn should_store_u32() {
        let (value1, value2): (u32, u32) = (1u32, 3u32);
        let (ret1, ret2) = generic_test::<u32>("store_u32", "test_u32", value1, value2);
        assert_eq!(value1, ret1);
        assert_eq!(value2, ret2);
    }

    #[test]
    fn should_store_u64() {
        let (value1, value2): (u64, u64) = (1u64, 3u64);
        let (ret1, ret2) = generic_test::<u64>("store_u64", "test_u64", value1, value2);
        assert_eq!(value1, ret1);
        assert_eq!(value2, ret2);
    }

    #[test]
    fn should_store_u128() {
        let (value1, value2): (U128, U128) = (U128::from(1), U128::from(3));
        let (ret1, ret2) = generic_test::<U128>("store_u128", "test_u128", value1, value2);
        assert_eq!(value1, ret1);
        assert_eq!(value2, ret2);
    }

    #[test]
    fn should_store_u256() {
        let (value1, value2): (U256, U256) = (U256::from(10), U256::from(30));
        let (ret1, ret2) = generic_test::<U256>("store_u256", "test_u256", value1, value2);
        assert_eq!(value1, ret1);
        assert_eq!(value2, ret2);
    }

    #[test]
    fn should_store_u512() {
        let (value1, value2): (U512, U512) = (U512::from(100), U512::from(300));
        let (ret1, ret2) = generic_test::<U512>("store_u512", "test_u512", value1, value2);
        assert_eq!(value1, ret1);
        assert_eq!(value2, ret2);
    }

    #[test]
    fn should_store_string() {
        let (value1, value2) = (String::from("hello"), String::from("world"));
        let (ret1, ret2) = generic_test::<String>(
            "store_string",
            "test_string",
            value1.clone(),
            value2.clone(),
        );
        assert_eq!(value1, ret1);
        assert_eq!(value2, ret2);
    }

    #[test]
    fn should_store_key() {
        let (value1, value2): (Key, Key) = (
            Key::Account(AccountHash::new([7u8; 32])),
            Key::Account(AccountHash::new([9u8; 32])),
        );
        let (ret1, ret2) = generic_test::<Key>("store_key", "test_key", value1, value2);
        assert_eq!(value1, ret1);
        assert_eq!(value2, ret2);
    }

    #[test]
    fn should_store_public_key() {
        let (value1, value2): (PublicKey, PublicKey) = (
            PublicKey::ed25519_from_bytes([1u8; 32]).unwrap(),
            PublicKey::ed25519_from_bytes([3u8; 32]).unwrap(),
        );
        let (ret1, ret2) =
            generic_test::<PublicKey>("store_public_key", "test_PublicKey", value1, value2);
        assert_eq!(value1, ret1);
        assert_eq!(value2, ret2);
    }

    #[test]
    fn should_store_option() {
        let (value1, value2): (Option<String>, Option<String>) =
            (Some(String::from("Hello")), None);
        let (ret1, ret2) =
            generic_test::<Option<String>>("store_option", "test_Option", value1.clone(), value2);
        assert_eq!(value1.unwrap(), ret1.unwrap());
        assert_eq!(ret2.is_none(), true);
    }

    #[test]
    fn should_store_list_of_bytes() {
        let (value1, value2): (Bytes, Bytes) = (
            vec![0x41u8, 0x41u8, 0x42u8].into(),
            vec![0x59u8, 0x59u8, 0x59u8].into(),
        );
        let (ret1, ret2) = generic_test::<Bytes>(
            "store_list_of_bytes",
            "test_list_of_bytes",
            value1.clone(),
            value2.clone(),
        );
        assert_eq!(value1, ret1);
        assert_eq!(value2, ret2);
    }

    #[test]
    fn should_store_byte_array() {
        let (value1, value2): ([u8; 3], [u8; 3]) = ([1, 2, 3], [2, 4, 6]);
        let (ret1, ret2) =
            generic_test::<[u8; 3]>("store_byte_array", "test_ByteArray", value1, value2);
        assert_eq!(value1, ret1);
        assert_eq!(value2, ret2);
    }

    #[test]
    fn should_store_result() {
        let (value1, value2): (Result<String, String>, Result<String, String>) =
            (Ok(String::from("Success")), Err(String::from("Fail")));
        let (ret1, ret2) = generic_test::<Result<String, String>>(
            "store_result",
            "test_Result",
            value1.clone(),
            value2,
        );
        assert_eq!(value1, ret1);
        assert_eq!(ret2.is_err(), true);
    }

    #[test]
    fn should_store_map() {
        let mut value1: BTreeMap<String, Option<String>> = BTreeMap::new();
        value1.insert(String::from("alice"), Some(String::from("Purchased Milk")));
        value1.insert(String::from("bob"), Some(String::from("Purchased Egg")));
        let mut value2: BTreeMap<String, Option<String>> = BTreeMap::new();
        value2.insert(String::from("alice"), Some(String::from("Sold Milk")));
        value2.insert(String::from("jane"), Some(String::from("Purchased Cake")));
        let (ret1, ret2) = generic_test::<BTreeMap<String, Option<String>>>(
            "store_map",
            "test_Map",
            value1.clone(),
            value2.clone(),
        );
        assert_eq!(value1, ret1);
        assert_eq!(value2, ret2);
    }

    #[test]
    fn should_store_tuple3() {
        let value1: (PublicKey, Option<String>, U512) = (
            PublicKey::ed25519_from_bytes([1u8; 32]).unwrap(),
            Some(String::from("Original")),
            U512::from(100),
        );
        let value2: (PublicKey, Option<String>, U512) = (
            PublicKey::ed25519_from_bytes([3u8; 32]).unwrap(),
            Some(String::from("Updated")),
            U512::from(300),
        );
        let (ret1, ret2) = generic_test::<(PublicKey, Option<String>, U512)>(
            "store_tuple3",
            "test_Tuple3",
            value1.clone(),
            value2.clone(),
        );
        assert_eq!(value1, ret1);
        assert_eq!(value2, ret2);
    }

    #[test]
    fn should_store_tuple2() {
        let value1: (String, U512) = (String::from("Original"), U512::from(100));
        let value2: (String, U512) = (String::from("Updated"), U512::from(300));
        let (ret1, ret2) = generic_test::<(String, U512)>(
            "store_tuple2",
            "test_Tuple2",
            value1.clone(),
            value2.clone(),
        );
        assert_eq!(value1, ret1);
        assert_eq!(value2, ret2);
    }
}
