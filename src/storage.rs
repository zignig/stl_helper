//! Ephermeral data storage for the server
//! This does not persist across restarts
//! based off https://tokio.rs/tokio/tutorial/shared-state
//!
//!     

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use crate::loader::View;

pub struct Storage {
    pub map: Arc<Mutex<HashMap<String, View>>>,
    pub data: Arc<Mutex<HashMap<String,Vec<u8>>>>
}

impl Storage {
    pub fn new() -> Self {
        Self {
            map: Arc::new(Mutex::new(HashMap::new())),
            data: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn clone(&mut self) -> Self { 
        Self { 
            map: self.map.clone(),
            data: self.data.clone()
        }
    }
}
