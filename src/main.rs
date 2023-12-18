mod data;
mod resource;

use axum::extract::{Json, Path, Query};
use axum::http::{header, StatusCode, Uri};
use axum::response::{AppendHeaders, Html, IntoResponse};
use axum::routing::{get, post};
use base64::engine::general_purpose;
use serde_json::{json, Value};
use std::collections::HashMap;
use std::thread;
use tokio::net::TcpListener;

use crate::data::DATA;

#[allow(unused)]
async fn print_data() {
    thread::spawn(move || {
        let data = DATA.lock().unwrap();
        println!("data: {:?}", data);
    })
    .join()
    .unwrap()
}

#[tokio::main]
async fn main() {
    // print_data().await;

    let app = axum::Router::new()
        .fallback(fallback)
        .route("/", get(hello))
        .route("/hello.html", get(get_hello_html))
        .route("/status", get(get_status))
        .route("/uri", get(get_uri))
        .route("/demo_png", get(get_demo_png))
        .route("/items", get(get_items))
        .route("/items/:id", get(get_items_id))
        .route("/demo.json", get(get_demo_json))
        .route("/demo.json", post(post_demo_json))
        .route("/resources", get(get_resources))
        .route("/demo.html", get(get_demo_html));

    let listener = TcpListener::bind("0.0.0.0:6969").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

pub async fn hello() -> String {
    "Hello World".into()
}

pub async fn fallback(uri: Uri) -> impl IntoResponse {
    (StatusCode::NOT_FOUND, format!("No route {}", uri))
}

pub async fn get_demo_html() -> Html<&'static str> {
    "<h1>Hello</h1>".into()
}

pub async fn get_hello_html() -> Html<&'static str> {
    include_str!("hello.html").into()
}

pub async fn get_status() -> (StatusCode, String) {
    (StatusCode::OK, "Everything is ok".to_string())
}

pub async fn get_uri(uri: Uri) -> String {
    format!("The URI is: {:?}", uri)
}

pub async fn get_demo_png() -> impl IntoResponse {
    use base64::Engine;
    let png = concat!(
        "iVBORw0KGgoAAAANSUhEUgAAAAEAAAAB",
        "CAYAAAAfFcSJAAAADUlEQVR42mPk+89Q",
        "DwADvgGOSHzRgAAAAABJRU5ErkJggg=="
    );
    (
        AppendHeaders([(header::CONTENT_TYPE, "image/png")]),
        general_purpose::STANDARD.decode(png).unwrap(),
    )
}

pub async fn get_items_id(Path(id): Path<String>) -> String {
    format!("Get items with path id: {:?}", id)
}

pub async fn get_items(Query(params): Query<HashMap<String, String>>) -> String {
    format!("Get items with query params: {:?}", params)
}

pub async fn get_demo_json() -> Json<Value> {
    json!({"a": "b"}).into()
}

pub async fn post_demo_json(axum::extract::Json(data): axum::Json<serde_json::Value>) -> String {
    format!("Post demo JSON data: {:?}\n", data)
}

pub async fn get_resources() -> axum::response::Html<String> {
    thread::spawn(move || {
        let data = DATA.lock().unwrap();
        let mut resources = data.values().collect::<Vec<_>>().clone();
        resources.sort_by(|a, b| a.name.cmp(&b.name));
        resources
            .iter()
            .map(|&resource| format!("<p>{}</p>\n", &resource))
            .collect::<String>()
    })
    .join()
    .unwrap()
    .into()
}
