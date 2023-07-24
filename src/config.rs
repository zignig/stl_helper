// config for  the server setup
use clap::{Parser, Subcommand};
use figment::{
    providers::{Format, Toml},
    Figment,
};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Parser,Debug)]
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

#[derive(Subcommand,Debug)]
pub enum Commands {
    Add,
    Remove,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub port: u16,
    pub folders: Vec<PathBuf>,
    pub autorotate: bool,
}

impl Config {
    pub fn new() -> Self {
        Self {
            port: 8001,
            folders: vec![PathBuf::from(r"incoming")],
            autorotate: false,
        }
    }

    pub fn startup() -> Self {
        let res = Figment::new().merge(Toml::file("config.toml")).extract();
        match res {
            Ok(conf) => {
                println!("{:#?}", conf);
                return conf;
            }
            Err(e) => {
                println!("error loading config {:?}", e);
                return Config::new();
            }
        }
    }

}

pub fn start() -> Config {
    let cli = Cli::parse();
    // do cli stuff
    println!("{:#?}",cli);
    let conf = Config::startup();

    conf
}
