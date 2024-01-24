#![deny(
    unsafe_code,
    clippy::correctness,
    clippy::nursery,
    clippy::pedantic,
    clippy::complexity,
    clippy::perf,
    clippy::style,
    clippy::suspicious
)]

use axum::{
    routing::get,
    Router,
};

use tokio::net::TcpListener;

#[tokio::main]
async fn main() {

    let x = 5_i32;

    let app = Router::new().route("/", get(root));

    let listener =  match TcpListener::bind("0.0.0.0:3000").await {
        Ok(x) => x,
        Err(e) => {
            println!("Error: {e}");
            return;
        }

    };



    match axum::serve(listener, app).await {
        Ok(x) => x,
        Err(e) => {
            println!("Error: {e}");
            return;
        }
    };
}


async fn root() -> &'static str {
    "Hello, World!"
}