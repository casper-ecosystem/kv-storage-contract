#[cfg(test)]
mod kv_storage;

#[cfg(test)]
mod tests {
    use super::kv_storage;
    use casper_types::account::AccountHash;
    use kv_storage::KVStorage;
    use kv_storage::KVSTORAGE_CONTRACT_NAME;
    use test_env::TestEnv;

    fn deploy() -> (TestEnv, KVStorage, AccountHash) {
        let env = TestEnv::new();
        let owner = env.next_user();
        let contract = KVStorage::new(&env, KVSTORAGE_CONTRACT_NAME, owner);
        (env, contract, owner)
    }

    #[test]
    fn should_create_string() {
        let (env, kv_storage, _) = deploy();
        let user = env.next_user();
        kv_storage.create(user, "Test", "string", Some(String::from("hello")));
        let result = kv_storage.read("Test", "string");
        assert!(result.is_some());
        assert_eq!(result.unwrap(), String::from("hello"));
    }

    #[test]
    fn should_update_string() {
        let (env, kv_storage, _) = deploy();
        let user = env.next_user();
        kv_storage.create(user, "Test", "string", Some(String::from("hello")));
        let result1 = kv_storage.read("Test", "string");
        assert!(result1.is_some());
        assert_eq!(result1.unwrap(), String::from("hello"));
        kv_storage.update(user, "Test", "string", Some(String::from("world")));
        let result2 = kv_storage.read("Test", "string");
        assert!(result2.is_some());
        assert_eq!(result2.unwrap(), String::from("world"));
    }

    #[test]
    fn should_delete_string() {
        let (env, kv_storage, _) = deploy();
        let user = env.next_user();
        kv_storage.create(user, "Test", "string", Some(String::from("hello")));
        let result1 = kv_storage.read("Test", "string");
        assert!(result1.is_some());
        assert_eq!(result1.unwrap(), String::from("hello"));
        kv_storage.delete(user, "Test", "string");
        let result2 = kv_storage.read("Test", "string");
        assert!(result2.is_none());
    }
}
