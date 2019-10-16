use std::collections::HashMap;

pub struct KvStore {
    container: HashMap<String, String>
}

impl KvStore
{
    pub fn default() -> KvStore
    {
        KvStore {
            container: HashMap::new()
        }
    }

    pub fn set(&mut self, key: String, value: String) {
        if self.container.contains_key(&key) {
            self.container.remove(&key);
        }
        self.container.insert(key, value);
    }
}