use lucid::kvstore::KvStore;

const CIPHER: std::option::Option<[&str; 2]> = Some([
    "123456789012345678901234123456789012345678901234",
    "f0f1f2f3f4f5f6f7f8f9fafbfcfdfeff",
]);

const DATA: [u8; 512] = [42u8; 512];

const KEY: &str = "test_value";

fn init_kv() -> KvStore {
    let kv = KvStore::new(CIPHER);
    kv.set(KEY.to_string(), DATA.to_vec(), None);
    kv
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_returns_a_value() {
        let kv = init_kv();
        let value = kv.get(KEY.to_string());

        match value {
            Some(v) => assert_eq!(v.data, DATA.to_vec()),
            None => panic!("No value found"),
        }
    }
}
