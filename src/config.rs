// config for  the server setup
use serde::{Serialize, Deserialize};
use serde_json::{Result, Value};
use std::path::PathBuf;

#[derive(Debug,Serialize,Deserialize)]
pub struct Config { 
    port: u16,
    folders: Vec<PathBuf>,
    autorotate: bool
}

impl Config { 
    pub fn new() -> Self{
        Self { 
            port: 8081,
            folders: vec!['incoming'],
            autorotate: false
        }
    }
}