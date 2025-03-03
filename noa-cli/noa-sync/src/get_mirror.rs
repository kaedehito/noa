use crate::{download_file::download_file, err};
use serde::{Deserialize, Serialize};
use std::{fs, path::Path};
use toml;

#[derive(Serialize, Deserialize)]
struct Mirror {
    mirrors: Vec<String>,
}

// Get the mirror url
// If not found /etc/noa/mirror/mirror.toml. download from https://noa.pik6c.tech/mirror.toml
pub fn get_mirror() -> Vec<String> {
    let path: &Path = "/etc/noa/mirror/mirror.toml".as_ref();

    // open file
    if !path.exists() {
        err!("/etc/noa/mirror/mirror.toml not fonud! downloading default mirror.toml..");

        download_file(
            "https://kaedehito.secret.jp/downloads/noa/mirror.toml",
            path,
        )
        .unwrap_or_else(|e| {
            err!("Failed to download mirror.toml: {e}");
            std::process::exit(1);
        });
    }

    let content = fs::read_to_string(path).unwrap_or_else(|e| {
        err!("Failed to read mirror.toml: {e}");
        std::process::exit(1);
    });

    let mirror: Mirror = toml::from_str(&content).unwrap_or_else(|e| {
        err!("Failed to parse mirror file: {}", e);
        std::process::exit(1);
    });

    mirror.mirrors
}
