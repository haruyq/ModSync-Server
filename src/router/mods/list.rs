use axum::{Json, Router, routing::get};
use serde_json as json;
use std::path::PathBuf;
use sha2::{Sha256, Digest};
use tokio::join;
use tokio::fs;
use tokio::fs::File;
use tokio::io::AsyncReadExt;

use crate::utils;
use crate::config;

pub fn router() -> Router {
    Router::new()
        .route("/mods/list", get(list_mods))
}

async fn get_sha256(path: &str) -> std::io::Result<String> {
    let mut file = File::open(path).await?;
    let mut hasher = Sha256::new();
    let mut buffer = [0u8; 8192];

    loop {
        let n = file.read(&mut buffer).await?;
        if n == 0 { break; }
        hasher.update(&buffer[..n]);
    }

    Ok(hex::encode(hasher.finalize()))
}

async fn scan_dir(dir: PathBuf) -> Vec<json::Value> {
    let mut result = Vec::new();

    if let Ok(mut entries) = fs::read_dir(dir).await {
        while let Ok(Some(entry)) = entries.next_entry().await {
            let path = entry.path();
            let needless = utils::is_needless(path.clone()).await;

            if path.is_file()
                && path.extension().and_then(|e| e.to_str()) == Some("jar")
                && !needless
            {
                let sha256 = get_sha256(path.to_str().unwrap_or(""))
                    .await
                    .unwrap_or_default();

                if let Some(file_name) = path.file_name().and_then(|n| n.to_str()) {
                    result.push(json::json!({
                        "name": file_name,
                        "sha256": sha256,
                    }));
                }
            }
        }
    }

    result
}

async fn list_mods() -> Json<json::Value> {
    let config = config::load_config();

    let mods_dir = PathBuf::from(&config.mods_dir);
    let deps_dir = PathBuf::from(&config.deps_dir);

    let (mods, deps) = join!(
        scan_dir(mods_dir),
        scan_dir(deps_dir)
    );

    let mut all = mods;
    all.extend(deps);

    Json(json::json!(all))
}