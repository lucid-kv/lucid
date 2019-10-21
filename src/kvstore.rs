use chashmap::CHashMap;

pub struct KvStore {
    container: CHashMap<String, String>
}

impl KvStore
{
    pub fn default() -> KvStore
    {
        KvStore {
            container: CHashMap::new()
        }
    }

    pub fn set(&self, key: String, value: String) -> Option<String> {
        self.container.insert(key, value)
    }

    pub fn get(&self, key: String) -> Option<String> {
        match (&self.container).get(&key) {
            Some(value) => {
                Some(value.to_string())
            },
            None => {
                None
            }
        }
    }

    pub fn drop(&self, key: String) {
        &self.container.remove(&key);
    }

    pub fn clear(&self) {
        &self.container.clear();
    }
}