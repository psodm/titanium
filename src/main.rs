mod resource;
mod data;

use serde_json::{json, Value};
use std::thread;

use crate::resource::Resource;

use crate::data::DATA;

async fn print_data() {
    thread::spawn(move || {
        let data = DATA.lock().unwrap();
        println!("data: {:?}", data);
    }).join().unwrap()
}

#[tokio::main]
async fn main() {
    // print_data().await;

    let app = axum::Router::new()
        .fallback(fallback)
        .route("/", axum::routing::get(hello))
        .route("/hello.html", axum::routing::get(get_hello_html))
        .route("/status", axum::routing::get(get_status))
        .route("/uri", axum::routing::get(get_uri))
        .route("/demo_png", axum::routing::get(get_demo_png))
        .route("/items", axum::routing::get(get_items))
        .route("/items/:id", axum::routing::get(get_items_id))
        .route("/demo.json", axum::routing::get(get_demo_json))
        .route("/demo.json", axum::routing::post(post_demo_json))
        .route("/resources", axum::routing::get(get_resources))
        .route("/demo.html", axum::routing::get(get_demo_html));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:6969").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

pub async fn hello() -> String {
    "Hello World".into()
}

pub async fn fallback(
    uri: axum::http::Uri
) -> impl axum::response::IntoResponse {
    (axum::http::StatusCode::NOT_FOUND, format!("No route {}", uri))
} 

pub async fn get_demo_html() -> axum::response::Html<&'static str> {
    "<h1>Hello</h1>".into()
}

pub async fn get_hello_html() -> axum::response::Html<&'static str> {
    include_str!("hello.html").into()
}

pub async fn get_status() -> (axum::http::StatusCode, String) {
    (axum::http::StatusCode::OK, "Everything is ok".to_string())
} 

pub async fn get_uri(uri: axum::http::Uri) -> String {
    format!("The URI is: {:?}", uri)
}

pub async fn get_demo_png() -> impl axum::response::IntoResponse {
    use base64::Engine;
    let png = concat!(
        "iVBORw0KGgoAAAANSUhEUgAAAAEAAAAB",
        "CAYAAAAfFcSJAAAADUlEQVR42mPk+89Q",
        "DwADvgGOSHzRgAAAAABJRU5ErkJggg=="
    );
    (
        axum::response::AppendHeaders([
            (axum::http::header::CONTENT_TYPE, "image/png"),
        ]),
        base64::engine::general_purpose::STANDARD.decode(png).unwrap(),
    )
}

pub async fn get_items_id(
    axum::extract::Path(id):
        axum::extract::Path<String>
) -> String {
    format!("Get items with path id: {:?}", id)
}

pub async fn get_items(
    axum::extract::Query(params):
        axum::extract::Query<std::collections::HashMap<String, String>>
) -> String {
    format!("Get items with query params: {:?}", params)
}

pub async fn get_demo_json() -> axum::extract::Json<Value> {
    json!({"a": "b"}).into()
}

pub async fn post_demo_json(
    axum::extract::Json(data):axum::Json<serde_json::Value>
) -> String {
    format!("Post demo JSON data: {:?}\n", data)
}

pub async fn get_resources() -> axum::response::Html<String> {
    thread::spawn(move || {
        let data = DATA.lock().unwrap();
        let mut resources = data.values().collect::<Vec<_>>().clone();
        resources.sort_by(|a, b| a.name.cmp(&b.name));
        resources.iter().map(|&resource|
            format!("<p>{}</p>\n", &resource)
        ).collect::<String>()
    }).join().unwrap().into()
}
