use axum::{
    Router, routing::get,
    extract::Path,
    response::Response,
    http::{StatusCode, header},
};
use tokio::fs::File;
use tokio_util::io::ReaderStream;
use std::path::PathBuf;

use crate::utils;

pub fn router() -> Router {
    Router::new().route("/mods/{name}", get(download))
}

async fn resolve_download_path(file: &str, mods_dir: &str, deps_dir: &str) -> Option<PathBuf> {
    let candidates = [
        PathBuf::from(mods_dir).join(file),
        PathBuf::from(deps_dir).join(file),
    ];

    for path in candidates {
        if path.exists() && !utils::is_needless(path.clone()).await {
            return Some(path);
        }
    }

    None
}

async fn download(Path(file): Path<String>) -> Result<Response, StatusCode> {
    if file.contains("..") {
        return Err(StatusCode::BAD_REQUEST);
    }

    let config = crate::config::load_config();
    let Some(path) = resolve_download_path(&file, &config.mods_dir, &config.deps_dir).await else {
        return Err(StatusCode::NOT_FOUND);
    };

    let opened = File::open(path).await.map_err(|_| StatusCode::NOT_FOUND)?;
    let stream = ReaderStream::new(opened);

    let body = axum::body::Body::from_stream(stream);

    Ok(Response::builder()
        .header(header::CONTENT_TYPE, "application/java-archive")
        .header(
            header::CONTENT_DISPOSITION,
            format!("attachment; filename=\"{}\"", file),
        )
        .body(body)
        .unwrap())
}
