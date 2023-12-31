#[macro_use]
extern crate rocket;

use rocket::response::stream::{Event, EventStream};
use rocket::tokio::select;
use rocket::tokio::sync::broadcast::{channel, error::RecvError, Receiver, Sender};
use rocket::tokio::time::{interval_at, Instant};
use rocket::{tokio, Config, Shutdown, State};

mod config;
mod loader;
mod storage;
mod watcher;

use loader::View;

use crate::storage::Storage;

use askama::Template;

use rocket::http::ContentType;
use rocket::response::content::RawHtml;
use rust_embed::RustEmbed;

use std::borrow::Cow;
use std::ffi::OsStr;
use std::path::PathBuf;

// Baked in assets
#[derive(RustEmbed)]
#[folder = "static"]
struct Asset;

/// Serve Baked Assets
#[get("/static/<file..>")]
async fn baked(file: PathBuf) -> Option<(ContentType, Cow<'static, [u8]>)> {
    let filename = file.display().to_string();
    let asset = Asset::get(&filename)?;
    let content_type = file
        .extension()
        .and_then(OsStr::to_str)
        .and_then(ContentType::from_extension)
        .unwrap_or(ContentType::Bytes);
    Some((content_type, asset.data))
}
/// App.js template
/// Add fetures they come up.
#[derive(Template)]
#[template(path = "app.js", escape = "none")]
struct AppJsTmpl;

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTmpl{
    recent: Vec<String>
}

/// Returns an infinite stream of server-sent events. Each event is a message
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
            // Move last mesag into state
            // todo borked.
            //recent = State::from(msg);
            yield Event::json(&msg);
        }
    }
}

#[get("/")]
async fn index(store: &State<Storage>) -> (ContentType,String) {
    let lru_list = store.map.lock().unwrap();
    let mut list:Vec<String> = Vec::new();
    for (i, _ ) in lru_list.iter(){ 
        list.push(i.to_string())
    }
    let data = IndexTmpl{
        recent: list
    };
    (ContentType::HTML,data.render().unwrap())
}

#[get("/app.js")]
async fn app_js() -> (ContentType, String) {
    let data = AppJsTmpl {};
    (ContentType::JavaScript, data.render().unwrap())
}

#[get("/recent/<name>")]
async fn recent(name: String, tx: &State<Sender<View>>, store: &State<Storage>) {
    let mut map = store.map.lock().unwrap();
    if let Some(view) = map.get(&name) {
        let mut cview = view.clone();
        for (i, _) in map.iter() {
            cview.recent.push(i.to_string());
            //println!("{:#?}", i)
        }
        let _ = tx.send(cview);
    }
}

#[get("/model/<name>")]
async fn model(name: String, store: &State<Storage>) -> Option<Vec<u8>> {
    let mut map = store.data.lock().unwrap();
    if let Some(data) = map.get(&name) {
        return Some(data.to_vec());
    }
    None
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let conf = config::start();

    // Create the primary channel
    let (tx, rx) = channel::<View>(1024);
    let rocket_tx = tx.clone();

    // Create the storagee
    let mut stor = storage::Storage::new();
    let other = stor.clone();

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
        let _ = watcher::async_debounce_watch(other, tx, conf.folders).await;
    });

    // Web Config
    let config = Config {
        port: conf.port,
        address: std::net::Ipv4Addr::new(0, 0, 0, 0).into(),
        workers: 10,
        ..Config::debug_default()
    };

    // Web Server
    rocket::custom(&config)
        .manage(rx)
        .manage(stor)
        .manage(rocket_tx)
        .mount("/", routes![index, app_js, recent, baked, events, model])
        //.mount("/", FileServer::from(relative!("static")))
        .launch()
        .await?;
    Ok(())
}
