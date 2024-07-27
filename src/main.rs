use axum::{response::Html, routing::get, Router};

#[tokio::main]
async fn main() {
    // single router
    let app =Router::new().route("/", get(handler));

    let listener=tokio::net::TcpListener::bind("127.0.0.1:8000")
    .await
    .unwrap();

    println!("Listening on: {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}


async fn handler() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}
