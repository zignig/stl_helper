// Async folder watcher that sends a message

use async_watcher::{notify::RecursiveMode, AsyncDebouncer};
use rocket::tokio::sync::broadcast::Sender;
use std::{path::Path, time::Duration};

use crate::loader;
use crate::loader::View;

// #[tokio::main]
// async fn main() -> Result<(), Box<dyn std::error::Error>> {
//     let paths = vec!["Cargo.toml", "Cargo.lock", "crates", "examples"];

//     async_debounce_watch(paths).await?;

//     Ok(())
// }
pub async fn async_debounce_watch<P: AsRef<Path>>(
    sender: Sender<loader::View>,
    paths: Vec<P>,
) -> Result<(), Box<dyn std::error::Error>> {
    let (tx, mut rx) = tokio::sync::mpsc::channel(100);

    let mut debouncer =
        AsyncDebouncer::new(Duration::from_secs(1), Some(Duration::from_millis(100)), tx).await?;

    paths.iter().for_each(|p| {
        debouncer
            .watcher()
            .watch(p.as_ref(), RecursiveMode::Recursive)
            .unwrap();
    });

    while let Some(event) = rx.recv().await {
        match event {
            Ok(events) => {
                events.iter().for_each(|e| {
                    println!("event: {:?}", e);
                    let f = &e.path;
                    // Does the file exist
                    if f.exists() {
                        if let Some(ext) = f.extension() {
                            if let Some(exts) = ext.to_str() {
                                match exts {
                                    "stl" => {
                                        println!("process stl");
                                        if let Some(view) = loader::process(f){
                                            sender.send(view); 
                                        }                                       
                                    }
                                    _ => println!("no binding"),
                                }
                            }
                        }
                    }
                });
            }
            Err(errors) => {
                for error in errors {
                    println!("error: {error:?}");
                }
            }
        }
    }

    Ok(())
}
