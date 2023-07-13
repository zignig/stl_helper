#[macro_use] extern crate rocket;

use rocket::tokio::time::{Instant, interval_at};
use rocket::{State, Shutdown, tokio, Config};
use rocket::fs::{relative, FileServer};
use rocket::form::Form;
use rocket::response::stream::{EventStream, Event};
use rocket::serde::{Serialize, Deserialize};
use rocket::tokio::sync::broadcast::{channel, Sender,Receiver, error::RecvError};
use rocket::tokio::select;

mod watcher;
mod loader;
use loader::View;

/// Returns an infinite stream of server-sent events. Each event is a message
/// pulled from a broadcast queue sent by the `post` handler.
/// old 
#[get("/events")]
async fn events(queue: &State<Receiver<View>>, mut end: Shutdown) -> EventStream![] {
    let mut rx = queue.resubscribe();
    EventStream! {
        loop {
            let msg = select! {
                msg = rx.recv() => match msg {
                    Ok(msg) => msg,
                    Err(RecvError::Closed) => break,
                    Err(RecvError::Lagged(_)) => continue,
                },
                _ = &mut end => break,
            };
            yield Event::json(&msg);
        }
    }
}

#[rocket::main] 
async fn main() ->  Result<(), rocket::Error> {
    // Create the primary channel
    let (tx,mut rx) = channel::<View>(1024);
    
    // Cleanup
    tokio::spawn(async {
        let start = Instant::now();
        let mut interval = interval_at(start, tokio::time::Duration::from_secs(60));

        loop {
            interval.tick().await;
            println!("File cleanup ");
        }
    });

    // File change
    tokio::spawn( async {
        let _ = watcher::async_debounce_watch(tx,vec!["/opt/opencascade-rs/crates/opencascade-sys/"]).await;
    });

    // Web Config
    let config = Config {
        port: 8001,
        address: std::net::Ipv4Addr::new(0, 0, 0, 0).into(),
        ..Config::debug_default()
    };

    // Web Server
    rocket::custom(&config)
        .manage(rx)
        .mount("/", routes![events])
        .mount("/", FileServer::from(relative!("static")))
        .launch()
        .await?;
    Ok(())
}