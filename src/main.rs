#[macro_use]
extern crate rocket;

use rocket::form::Form;
use rocket::fs::{relative, FileServer};
use rocket::response::stream::{Event, EventStream};
use rocket::serde::{Deserialize, Serialize};
use rocket::tokio::select;
use rocket::tokio::sync::broadcast::{channel, error::RecvError, Receiver, Sender};
use rocket::tokio::time::{interval_at, Instant};
use rocket::{tokio, Config, Shutdown, State};

mod loader;
mod watcher;
mod config;

use loader::View;
use tokio::sync::Mutex;

/// Returns an infinite stream of server-sent events. Each event is a message
/// pulled from a broadcast queue sent by the `post` handler.
/// old
#[get("/events")]
async fn events(queue: &State<Receiver<View>>,recent: &State<View>, mut end: Shutdown) -> EventStream![] {
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
            // Move last mesag into state 
            // todo borked.
            //recent = State::from(msg);
            yield Event::json(&msg);
        }
    }
}

// Save the last view ( need to manage in another type)
type RecentView = Mutex<View>;


#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    // Create the primary channel
    let (tx, mut rx) = channel::<View>(1024);

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
    tokio::spawn(async {
        let _ =
            watcher::async_debounce_watch(tx, vec!["/opt/viewer/stls/", "/opt/opencascade-rs/"])
                .await;
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
        .manage(View::new())
        .mount("/", routes![events])
        .mount("/", FileServer::from(relative!("static")))
        .launch()
        .await?;
    Ok(())
}
