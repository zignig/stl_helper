//! Ephermeral data storage for the server
//! This does not persist across restarts
//! based off https://tokio.rs/tokio/tutorial/shared-state
//!
//!     

use std::sync::{Arc, Mutex};

use crate::loader::View;

extern crate lru;

use lru::LruCache;
use std::num::NonZeroUsize;

static CACHE_SIZE:usize = 20;

pub struct Storage {
    pub map: Arc<Mutex<LruCache<String, View>>>,
    pub data: Arc<Mutex<LruCache<String,Vec<u8>>>>
}

impl Storage {
    pub fn new() -> Self {
        Self {
            map: Arc::new(Mutex::new(LruCache::new(NonZeroUsize::new(CACHE_SIZE.into()).unwrap()))),
            data: Arc::new(Mutex::new(LruCache::new(NonZeroUsize::new(CACHE_SIZE.into()).unwrap()))),
        }
    }

    pub fn clone(&mut self) -> Self { 
        Self { 
            map: self.map.clone(),
            data: self.data.clone()
        }
    }
}
