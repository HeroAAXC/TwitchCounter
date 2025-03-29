const HTML: &str = include_str!("./index.html");

use std::sync::Arc;

use axum::{extract::State, response::Html, routing::get, Router};
use tokio::sync::RwLock;

use crate::counter_data::CounterData;

pub async fn init_web(data: Arc<RwLock<Option<CounterData>>>) {
    let app = Router::new().route("/", get(root).with_state(data));

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
