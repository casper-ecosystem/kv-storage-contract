#![cfg(test)]

mod kv_storage;

mod tests {
    use super::kv_storage;
    use casperlabs_types::{account::PublicKey, U512};
    use kv_storage::KVstorageContract;

    #[test]
    fn should_store_u64() {
        const KEY_NAME: &str = "testu64";
        const KEY_TYPE: &str = "u64";
        let mut kv_storage = KVstorageContract::deploy();
        let args = (KEY_NAME, KEY_TYPE, 1u64);
        kv_storage.call_indirect(args);
        let value: u64 = kv_storage.query_contract(&KEY_NAME).unwrap();
        assert_eq!(value, 1);
    }

    #[test]
    fn should_store_string() {
        const KEY_NAME: &str = "teststring";
        const KEY_TYPE: &str = "string";
        let mut kv_storage = KVstorageContract::deploy();
        let args = (KEY_NAME, KEY_TYPE, String::from("Hello World"));
        kv_storage.call_indirect(args);
        let value: String = kv_storage.query_contract(&KEY_NAME).unwrap();
        assert_eq!(value, String::from("Hello World"));
    }

    #[test]
    fn should_store_u512() {
        const KEY_NAME: &str = "testU512";
        const KEY_TYPE: &str = "U512";
        let mut kv_storage = KVstorageContract::deploy();
        let args = (KEY_NAME, KEY_TYPE, U512::from(100));
        kv_storage.call_indirect(args);
        let value: U512 = kv_storage.query_contract(&KEY_NAME).unwrap();
        assert_eq!(value, U512::from(100));
    }

    #[test]
    fn should_store_public_key() {
        const BOB_ACCOUNT: PublicKey = PublicKey::ed25519_from([7u8; 32]);
        const KEY_NAME: &str = "testpublickey";
        const KEY_TYPE: &str = "public_key";
        let mut kv_storage = KVstorageContract::deploy();
        let args = (KEY_NAME, KEY_TYPE, BOB_ACCOUNT);
        kv_storage.call_indirect(args);
        let value: PublicKey = kv_storage.query_contract(&KEY_NAME).unwrap();
        assert_eq!(value, BOB_ACCOUNT);
    }

    #[test]
    fn should_update_u64() {
        const KEY_NAME: &str = "testu64";
        const KEY_TYPE: &str = "u64";
        let mut kv_storage = KVstorageContract::deploy();
        let args = (KEY_NAME, KEY_TYPE, 1u64);
        kv_storage.call_indirect(args);
        let update_args = (KEY_NAME, KEY_TYPE, 2u64);
        kv_storage.call_indirect(update_args);
        let value: u64 = kv_storage.query_contract(&KEY_NAME).unwrap();
        assert_eq!(value, 2);
    }

    #[test]
    fn should_update_string() {
        const KEY_NAME: &str = "teststring";
        const KEY_TYPE: &str = "string";
        let mut kv_storage = KVstorageContract::deploy();
        let args = (KEY_NAME, KEY_TYPE, String::from("Hello World"));
        kv_storage.call_indirect(args);
        let update_args = (KEY_NAME, KEY_TYPE, String::from("Goodbye friend"));
        kv_storage.call_indirect(update_args);
        let value: String = kv_storage.query_contract(&KEY_NAME).unwrap();
        assert_eq!(value, String::from("Goodbye friend"));
    }

    #[test]
    fn should_update_u512() {
        const KEY_NAME: &str = "testU512";
        const KEY_TYPE: &str = "U512";
        let mut kv_storage = KVstorageContract::deploy();
        let args = (KEY_NAME, KEY_TYPE, U512::from(100));
        kv_storage.call_indirect(args);
        let update_args = (KEY_NAME, KEY_TYPE, U512::from(200));
        kv_storage.call_indirect(update_args);
        let value: U512 = kv_storage.query_contract(&KEY_NAME).unwrap();
        assert_eq!(value, U512::from(200));
    }
    #[test]
    fn should_update_public_key() {
        const BOB_ACCOUNT: PublicKey = PublicKey::ed25519_from([7u8; 32]);
        const ALI_ACCOUNT: PublicKey = PublicKey::ed25519_from([3u8; 32]);
        const KEY_NAME: &str = "testpublickey";
        const KEY_TYPE: &str = "public_key";
        let mut kv_storage = KVstorageContract::deploy();
        let args = (KEY_NAME, KEY_TYPE, BOB_ACCOUNT);
        kv_storage.call_indirect(args);
        let update_args = (KEY_NAME, KEY_TYPE, ALI_ACCOUNT);
        kv_storage.call_indirect(update_args);
        let value: PublicKey = kv_storage.query_contract(&KEY_NAME).unwrap();
        assert_eq!(value, ALI_ACCOUNT);
    }
}
