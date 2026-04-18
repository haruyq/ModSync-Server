use tokio::fs;
use std::path::PathBuf;

const NEEDLESS_LIST_PATH: &str = "./needless.txt";

async fn load_needless_list() -> Vec<String> {
    let needless_list = fs::read_to_string(NEEDLESS_LIST_PATH).await.unwrap_or_default();

    needless_list
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty() && !line.starts_with('#'))
        .map(|line| line.to_string())
        .collect()
}

pub async fn is_needless(path: PathBuf) -> bool {
    let needless_list = load_needless_list().await;
    let path_str = path.to_string_lossy();
    let file_name = path.file_name().and_then(|name| name.to_str());

    needless_list.iter().any(|line| {
        line == path_str.as_ref() || file_name.map(|name| name == line).unwrap_or(false)
    })
}