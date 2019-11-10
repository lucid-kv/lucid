use chashmap::CHashMap;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct KvElement {
    pub data: Vec<u8>,
    pub mime: String,
    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
    pub locked: bool
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
        match &mut self.container.get_mut(&key) {
            Some(kv_element) => {
                kv_element.updated = Utc::now();
                Some(kv_element.to_owned())
            },
            None => {
                let mime_type = tree_magic::from_u8(value.as_ref());
                let kv_element = KvElement {
                    data: value,
                    mime: mime_type,
                    created: Utc::now(),
                    updated: Utc::now(),
                    locked: false,
                };
                self.container.insert(key, kv_element)
            }
        }
    }

    pub fn get(&self, key: String) -> Option<Vec<u8>> {
        match &self.container.get(&key) {
            Some(value) => Some((&value.data).clone()),
            None => None
        }
    }

    // TODO: implement Lock, Unlock, Increment, Decrement, Expire

    pub fn drop(&self, key: String) {
        &self.container.remove(&key);
    }
}