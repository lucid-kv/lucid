use block_modes::block_padding::ZeroPadding;
use block_modes::{BlockMode, Cbc};
use chashmap::CHashMap;
use chrono::{DateTime, Duration, Utc};

use serpent::Serpent;

type SerpentCbc = Cbc<Serpent, ZeroPadding>;

#[derive(Debug, Clone)]
pub struct KvElement {
    pub data: Vec<u8>,
    pub mime_type: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub expire_at: DateTime<Utc>,
    pub update_count: i32,
    pub locked: bool,
}

pub struct KvStore {
    container: CHashMap<String, KvElement>,
    cipher: Option<Cipher>,
}

pub struct Cipher {
    priv_key: [u8; 24],
    iv: [u8; 16],
}

impl KvStore {
    pub fn new(cipher: Option<[&str; 2]>) -> KvStore {
        // TODO: prepare looped persistence
        let mut kv = KvStore {
            container: CHashMap::new(),
            cipher: None,
        };

        if let Some(c) = cipher {
            let (mut priv_key, mut iv) = ([0u8; 24], [0u8; 16]);
            priv_key[..24].copy_from_slice(&hex::decode(c[0]).unwrap());
            iv[..16].copy_from_slice(&hex::decode(c[1]).unwrap());
            kv.cipher = Some(Cipher { priv_key, iv });
        }

        kv
    }

    pub fn set(&self, key: String, mut value: Vec<u8>, mime: Option<String>) -> Option<KvElement> {
        // TODO: prepare iterative persistence
        if let Some(c) = &self.cipher {
            let cipher = SerpentCbc::new_var(&c.priv_key, &c.iv).unwrap();
            value = cipher.encrypt_vec(&value);
        }
        let mime_type = match mime {
            Some(gived_mimetype) => gived_mimetype,
            None => tree_magic::from_u8(value.as_ref()).to_string(),
        };
        match &mut self.container.get_mut(&key) {
            Some(kv_element) => {
                if !kv_element.locked {
                    kv_element.data = value;
                    kv_element.mime_type = mime_type;
                }
                kv_element.updated_at = Utc::now();
                kv_element.update_count = kv_element.update_count + 1;
                Some(kv_element.to_owned())
            }
            None => {
                let kv_element = KvElement {
                    data: value,
                    mime_type,
                    created_at: Utc::now(),
                    updated_at: Utc::now(),
                    expire_at: Utc::now(),
                    update_count: 1,
                    locked: false,
                };
                self.container.insert(key, kv_element)
            }
        }
    }

    pub fn get(&self, key: String) -> Option<KvElement> {
        match self.container.get(&key) {
            Some(value) => {
                let mut cloned_value = value.clone();

                if let Some(c) = &self.cipher {
                    let cipher = SerpentCbc::new_var(&c.priv_key, &c.iv).unwrap();
                    cloned_value.data = cipher.decrypt_vec(&value.data).unwrap();
                }
                Some(cloned_value)
            }
            None => None,
        }
    }

    pub fn switch_lock(&self, key: String, to_lock: bool) -> bool {
        match &mut self.container.get_mut(&key) {
            Some(kv_element) => {
                if kv_element.locked == to_lock {
                    return false;
                }
                kv_element.locked = to_lock;
                true
            }
            None => false,
        }
    }

    pub fn increment_or_decrement(&self, key: String, value: f64) -> bool {
        match &mut self.container.get_mut(&key) {
            Some(kv_element) => {
                let byte_to_string = String::from_utf8(kv_element.clone().data).unwrap(); // TODO: handle convert to string error
                match byte_to_string.trim().parse::<f64>() {
                    Ok(initial_value) => {
                        kv_element.data = (initial_value + value).to_string().into_bytes();
                        kv_element.updated_at = Utc::now();
                        kv_element.update_count = kv_element.update_count + 1;
                        true
                    }
                    Err(_) => false,
                }
            }
            None => false,
        }
    }

    pub fn set_expiration(&self, key: String, ttl: i64) -> Option<DateTime<Utc>> {
        match &mut self.container.get_mut(&key) {
            Some(kv_element) => {
                let expiration_date = Utc::now() + Duration::seconds(ttl);
                kv_element.expire_at = expiration_date;
                kv_element.updated_at = Utc::now();
                kv_element.update_count = kv_element.update_count + 1;
                Some(expiration_date)
            }
            None => None,
        }
    }

    pub fn drop(&self, key: String) {
        self.container.remove(&key);
    }
}
