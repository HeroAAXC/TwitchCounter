use std::collections::HashSet;
use std::fmt::Display;
use std::str::Split;
use std::sync::Arc;

use axum::extract::State;
use axum::response::Html;
use axum::routing::get;
use axum::Router;
use tokio::join;
use tokio::sync::{Mutex, RwLock};
use tokio::time::Instant;
use twitch_irc::login::StaticLoginCredentials;
use twitch_irc::message::{PrivmsgMessage, ServerMessage};
use twitch_irc::TwitchIRCClient;
use twitch_irc::{ClientConfig, SecureTCPTransport};

const HTML: &str = include_str!("../index.html");

#[tokio::main]
pub async fn main() {
    let mods = {
        let mut temp = HashSet::new();

        for line in std::fs::read_to_string("./mods.csv").unwrap().split("\n") {
            let name = line
                .replace("\t", "")
                .replace(" ", "")
                .replace("\n", "")
                .to_lowercase();
            temp.insert(name);
        }
        temp
    };

    let counter: Arc<RwLock<Option<CounterData>>> = Arc::new(RwLock::new(None));
    let counter_clone = counter.clone();

    let config = ClientConfig::default();
    let (mut incoming_messages, client) =
        TwitchIRCClient::<SecureTCPTransport, StaticLoginCredentials>::new(config);

    let join_handle = tokio::spawn(async move {
        while let Some(message) = incoming_messages.recv().await {
            if let ServerMessage::Privmsg(msg) = message {
                recv_message(msg, &mods, &Arc::from(&counter)).await;
            }
        }
    });

    client.join("misterjp1987".to_owned()).unwrap();

    let web_future = init_web(counter_clone);
    join!(join_handle, web_future).0.unwrap();
}

fn parse_to_secs(time: &str) -> Option<u64> {
    let mut s: Split<'_, &str> = time.split(":");
    let minutes: u64 = match s.next()?.parse() {
        Ok(s) => s,
        Err(_) => return None,
    };

    let secs = match s.next() {
        Some(s) => match s.parse() {
            Ok(s) => s,
            Err(_) => return None,
        },
        None => 0u64,
    };
    Some(minutes * 60u64 + secs)
}

async fn recv_message(
    msg: PrivmsgMessage,
    mods: &HashSet<String>,
    counter: &Arc<RwLock<Option<CounterData>>>,
) {
    if mods.contains(&msg.sender.name.to_lowercase()) && msg.message_text.starts_with("!counter ") {
        let mut time = msg.message_text.split(" ");
        time.next();
        if let Some(time) = time.next() {
            let secs = parse_to_secs(time);
            let mut counter = counter.write().await;
            if let Some(s) = secs {
                *counter = Some(CounterData::new(s));
                println!("set counter");
            }
        }
    }
}

async fn init_web(data: Arc<RwLock<Option<CounterData>>>) {
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root).with_state(data));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root(State(data): State<Arc<RwLock<Option<CounterData>>>>) -> Html<String> {
    let replace_with = match data.read().await.clone() {
        Some(s) => format!("{}", s),
        None => "".to_owned(),
    };

    Html(HTML.replacen("{}", replace_with.as_str(), 1))
}

#[derive(Copy, Clone, Debug)]
pub struct CounterData {
    pub secs: u64,
    pub from: Instant,
}

impl Display for CounterData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let elapsed = self.from.elapsed().as_secs();
        let elapsed = if elapsed > self.secs {
            0
        } else {
            self.secs - elapsed
        };
        let minutes = elapsed / 60;
        let secs = elapsed % 60;
        write!(f, "{}:{}", minutes, secs)
    }
}

impl CounterData {
    pub fn new(secs: u64) -> Self {
        Self {
            secs,
            from: Instant::now(),
        }
    }
}
