use chashmap::CHashMap;
use chrono::{DateTime, Utc};

pub struct KvElement {
    data: Vec<u8>,
    created: DateTime<Utc>,
}

pub struct KvStore {
    container: CHashMap<String, KvElement>
}

impl KvStore
{
    pub fn default() -> KvStore
    {
        KvStore {
            container: CHashMap::new()
        }
    }

    pub fn set(&self, key: String, value: Vec<u8>) -> Option<KvElement> {
        self.container.insert(key, KvElement { data: value, created: Utc::now() })
    }

    pub fn get(&self, key: String) -> Option<Vec<u8>> {
        match (&self.container).get(&key) {
            Some(value) => Some((&value.data).clone()),
            None => None
        }
    }

    pub fn drop(&self, key: String) {
        &self.container.remove(&key);
    }
}