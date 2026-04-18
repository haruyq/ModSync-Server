use axum::Router;
use axum::middleware as mw;

mod config;
mod router {
    pub mod health;
    pub mod mods;
}
mod middleware {
    pub mod secret;
}

fn create_app() -> Router {
    Router::new()
        .merge(router::health::router())
        .merge(router::mods::list::router())
        .merge(router::mods::download::router())
        
        .layer(mw::from_fn(middleware::secret::api_secret))
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