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

#[derive(Debug, Clone, FromForm, Serialize, Deserialize)]
#[cfg_attr(test, derive(PartialEq, UriDisplayQuery))]
#[serde(crate = "rocket::serde")]
struct Message {
    #[field(validate = len(..30))]
    pub room: String,
    #[field(validate = len(..20))]
    pub username: String,
    pub message: String,
}

/// Returns an infinite stream of server-sent events. Each event is a message
/// pulled from a broadcast queue sent by the `post` handler.
/// old 
#[get("/events")]
//async fn events(queue: &State<Sender<Message>>, mut end: Shutdown) -> EventStream![] {
async fn events(queue: &State<Receiver<String>>, mut end: Shutdown) -> EventStream![] {
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

/// Receive a message from a form submission and broadcast it to any receivers.
#[post("/message", data = "<form>")]
fn post(form: Form<Message>, queue: &State<Sender<Message>>) {
    // A send 'fails' if there are no active subscribers. That's okay.
    let _res = queue.send(form.into_inner());
}

// #[launch]
// fn rocket() -> _ {
// }
// struct STLLoader { 
//     rx: Sender<String>
// }



#[rocket::main] 
async fn main() ->  Result<(), rocket::Error> {
    // Create the primary channel
    let (tx,mut rx) = channel::<String>(1024);
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
        let _ = watcher::async_debounce_watch(tx,vec!["/opt/viewer/static/models"]).await;
    });

    // Web Config
    let config = Config {
        port: 8001,
        address: std::net::Ipv4Addr::new(0, 0, 0, 0).into(),
        ..Config::debug_default()
    };
    //let incoming = STLLoader{ rx:rx};
    // Web Server
    rocket::custom(&config)
        .manage(channel::<Message>(1024).0)
        .manage(rx)
        .mount("/", routes![post, events])
        .mount("/", FileServer::from(relative!("static")))
        .launch()
        .await?;
    Ok(())
}