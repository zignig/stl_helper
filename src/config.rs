use std::{fs, process::exit};
// config for  the server setup
use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use serde_json::{Result, Value};
use std::path::PathBuf;
use toml;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Optional name to operate on
    name: Option<String>,

    /// Sets a custom config file
    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,

    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    Add,
    Remove,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    port: u16,
    folders: Vec<PathBuf>,
    autorotate: bool,
}

impl Config {
    pub fn new() -> Self {
        Self {
            port: 8081,
            folders: vec![PathBuf::from(r"incoming")],
            autorotate: false,
        }
    }

    pub fn load(filename: String) -> Self {
        let settings = match fs::read_to_string(filename) {
            Ok(c) => c,
            Err(_) => exit(1),
        };

        // Load the config file from toml
        let data: Config = match toml::from_str(&settings) {
            Ok(d) => d,
            Err(e) => exit(1),
        };
        data
    }

    pub fn save(&mut self, filename: String) {}
}
