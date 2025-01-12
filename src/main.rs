use axum::{routing::get, Router, Json, serve};
use serde::Serialize;

#[derive(Serialize)]
struct Message {
    text: String,
}

async fn handle_request() -> Json<Message> {
    Json(Message { text: "Hello world!".to_string() })
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(handle_request));
    
    // Create a TCP listener
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Server running on http://0.0.0.0:3000");
    
    // Start the server
    serve(listener, app).await.unwrap();
}