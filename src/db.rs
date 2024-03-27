use anyhow::{bail, Result};
use bytes::Bytes;
use std::{collections::HashMap, sync::Arc};
use tokio::sync::Mutex;

struct Entry {
    data: Bytes,
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
        Ok(val.data.clone())
    }

    pub async fn set(&self, key: String, data: Bytes) {
        let mut map = self.shared.lock().await;
        map.insert(key, Entry { data });
    }
}
