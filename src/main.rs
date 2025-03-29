use std::collections::HashSet;
use std::sync::Arc;

use config::{init_mods, load_channel};
use counter_data::{parse_to_secs, CounterData};
use tokio::join;
use tokio::sync::RwLock;
use twitch_irc::login::StaticLoginCredentials;
use twitch_irc::message::{PrivmsgMessage, ServerMessage};
use twitch_irc::TwitchIRCClient;
use twitch_irc::{ClientConfig, SecureTCPTransport};
use web::init_web;

mod config;
mod counter_data;
mod web;

#[tokio::main]
pub async fn main() {
    let (mods, channel) = join!(init_mods(), load_channel());

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

    client.join(channel).unwrap();

    join!(join_handle, init_web(counter_clone)).0.unwrap();
}

async fn recv_message(
    msg: PrivmsgMessage,
    mods: &HashSet<String>,
    counter: &Arc<RwLock<Option<CounterData>>>,
) {
    if mods.contains(&msg.sender.name.to_lowercase()) {
        if msg.message_text.starts_with("!counter ") {
            let mut time = msg.message_text.split(" ");
            time.next();
            if let Some(time) = time.next() {
                let secs = parse_to_secs(time);
                let mut counter = counter.write().await;
                if let Some(s) = secs {
                    *counter = Some(CounterData::new(s));
                }
            }
        } else if msg.message_text.starts_with("!stopcounter") {
            let mut counter = counter.write().await;
            *counter = None;
        }
    }
}
