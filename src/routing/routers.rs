// routers.rs
use axum::{response::Html, routing::get, Router};

pub fn create_router() -> Router {
    let inference = Router::new()
        .route("/", get(inference));

    Router::new()
        .route("/", get(handler))
        .nest("/inference", inference)
}

async fn handler() -> Html<&'static str> {
    Html("<h1>Welcome!</h1>")
}

async fn inference() -> Html<&'static str> {
    Html("<h1>Inference!</h1>")
}