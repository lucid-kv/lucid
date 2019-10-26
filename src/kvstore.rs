use chashmap::CHashMap;

//use chrono::{DateTime, Utc};

pub struct KvElement {
    data: Vec<u8>,
//    created: DateTime<Utc>,
//    locked: bool
}

pub struct KvStore {
    container: CHashMap<String, KvElement>
}

impl KvStore
{
    pub fn default() -> KvStore
    {
        // TODO: prepare looped persistence
        KvStore {
            container: CHashMap::new()
        }
    }

    pub fn set(&self, key: String, value: Vec<u8>) -> Option<KvElement> {
        // TODO: prepare iterative persistence
        self.container.insert(key, KvElement { data: value })
    }

    pub fn get(&self, key: String) -> Option<Vec<u8>> {
        match (&self.container).get(&key) {
            Some(value) => Some((&value.data).clone()),
            None => None
        }
    }

    // TODO: implement Lock, Unlock, Increment, Decrement

    pub fn drop(&self, key: String) {
        &self.container.remove(&key);
    }
}