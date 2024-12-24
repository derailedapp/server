use axum::http::Method;
use tokio::net::TcpListener;
use tower_http::cors::{Any, CorsLayer};

#[tokio::main]
async fn main() {
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::DELETE, Method::PATCH])
        .allow_headers(Any)
        .allow_origin(Any);

    let app = axum::Router::new()
        .layer(cors);

    // keep consistency with port numbers
    let listener = TcpListener::bind("0.0.0.0:24640").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
