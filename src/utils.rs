use std::path::Path;
use tokio::fs::File;
use tokio::io::{AsyncBufReadExt, BufReader};

const NEEDLESS_LIST_PATH: &str = "./needless.txt";

pub async fn is_needless(path: &Path) -> bool {
    let path_str = path.to_string_lossy();
    let file_name = path.file_name().and_then(|name| name.to_str());

    if let Ok(needless_list) = File::open(NEEDLESS_LIST_PATH).await {
        let needless_list = BufReader::new(needless_list);

        let mut lines = needless_list.lines();

        while let Ok(Some(line)) = lines.next_line().await {
            let line = line.trim();

            if line.starts_with('#') {
                continue;
            }

            if line == path_str || file_name.map(|name| name == line).unwrap_or(false) {
                return true;
            }
        }
    }

    false
}