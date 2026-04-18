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

async fn download(Path(file): Path<String>) -> Result<Response, StatusCode> {
    let config = crate::config::load_config();
    let path = PathBuf::from(config.mods_dir).join(&file);

    if !path.exists() || utils::is_needless(path.clone()).await {
        return Err(StatusCode::NOT_FOUND);
    }

    if file.contains("..") {
        return Err(StatusCode::BAD_REQUEST);
    }

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
