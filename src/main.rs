use axum::Router;

mod routers;

fn create_app() -> Router {
    Router::new()
        .merge(routers::health::router())
}

#[tokio::main]
async fn main() {
    let app = create_app();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3456").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}