use axum::Router;

mod config;
mod router {
    pub mod health;
    pub mod mods;
}

fn create_app() -> Router {
    Router::new()
        .merge(router::health::router())
        .merge(router::mods::list::router())
        .merge(router::mods::download::router())
}

#[tokio::main]
async fn main() {
    let config = config::load_config();
    let app = create_app();
    let listener = tokio::net::TcpListener::bind(
        format!("0.0.0.0:{}", config.port)
    ).await.unwrap();

    axum::serve(listener, app).await.unwrap();
}