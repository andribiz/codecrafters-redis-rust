use anyhow::{bail, Result};
use bytes::Bytes;
use std::{
    collections::HashMap,
    sync::Arc,
    time::{Duration, Instant},
};
use tokio::sync::Mutex;

struct Entry {
    data: Bytes,
    expired_at: Option<Instant>,
}
pub type ArcDB = Arc<DB>;

pub struct DB {
    shared: Mutex<HashMap<String, Entry>>,
}

impl DB {
    pub fn new() -> Self {
        DB {
            shared: Mutex::new(HashMap::new()),
        }
    }

    pub async fn get(&self, key: String) -> Result<Bytes> {
        let map = self.shared.lock().await;
        let Some(val) = map.get(&key) else {
            bail!("key not found");
        };
        if let Some(expiry) = val.expired_at {
            if Instant::now() > expiry {
                bail!("key expired")
            }
        }
        Ok(val.data.clone())
    }

    pub async fn set(&self, key: String, data: Bytes, expiry: Option<u64>) {
        let mut map = self.shared.lock().await;
        let value = match expiry {
            Some(val) => Entry {
                data,
                expired_at: Instant::now().checked_add(Duration::from_millis(val)),
            },
            None => Entry {
                data,
                expired_at: None,
            },
        };
        map.insert(key, value);
    }
}
