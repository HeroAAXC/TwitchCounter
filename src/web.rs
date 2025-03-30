use std::sync::Arc;

use std::process;

use axum::{extract::State, response::Html, routing::get, Router};
use tokio::sync::RwLock;

use crate::counter_data::CounterData;

const EXIT_PAGE: &str = include_str!("./exit_page.html");
const HTML: &str = include_str!("./index.html");

pub async fn init_web(data: Arc<RwLock<Option<CounterData>>>) {
    let app = Router::new()
        .route("/", get(root).with_state(data))
        .route("/exit_message", get(exit_message))
        .route("/exit", get(exit_page));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root(State(data): State<Arc<RwLock<Option<CounterData>>>>) -> Html<String> {
    let html = HTML;
    let replace_with = match data.read().await.clone() {
        Some(s) => format!("{}", s),
        None => "".to_owned(),
    };

    let html = html.replacen(
        "{blinking}",
        match replace_with.ends_with("0") {
            true => "blinking",
            false => "other",
        },
        1,
    );

    Html(html.replacen("{}", replace_with.as_str(), 1))
}

async fn exit_message() -> Html<()> {
    process::exit(0x0100);
}

async fn exit_page() -> Html<&'static str> {
    Html(EXIT_PAGE)
}
