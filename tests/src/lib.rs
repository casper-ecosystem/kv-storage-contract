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
        kv_storage.create(user, "Test", "string", String::from("hello"));
        let result = kv_storage.read("Test", "string");
        assert_eq!(result, String::from("hello"));
    }

    #[test]
    fn should_update_string() {
        let (env, kv_storage, _) = deploy();
        let user = env.next_user();
        kv_storage.create(user, "Test", "string", String::from("hello"));
        let result1 = kv_storage.read("Test", "string");
        assert_eq!(result1, String::from("hello"));
        kv_storage.update(user, "Test", "string", String::from("world"));
        let result2 = kv_storage.read("Test", "string");
        assert_eq!(result2, String::from("world"));
    }

    #[test]
    fn should_delete_string() {
        let (env, kv_storage, _) = deploy();
        let user = env.next_user();
        kv_storage.create(user, "Test", "string", String::from("hello"));
        let result1 = kv_storage.read("Test", "string");
        assert_eq!(result1, String::from("hello"));
        kv_storage.delete(user, "Test", "string");
        let result2 = kv_storage.read("Test", "string");
        assert_eq!(result2, String::from(""));
    }
}
