use blake3::Hasher;
use std::fs;
use std::fs::File;
use std::io::{BufReader, Read};

use crate::download_file::download_file;
use crate::err;
use crate::get_mirror::get_mirror;

pub fn check_hash() -> Result<(), Box<dyn std::error::Error>> {
    let mirrors = get_mirror();
    for mirror in mirrors {
        let Ok(_) = download_file(
            &format!("{}/noa/packagelist.blake3", mirror),
            "/etc/noa/cache/packagelist.blake3",
        ) else{
            continue;
        };
    }
    let mut hasher = Hasher::new();
    let mut file = BufReader::new(File::open("/etc/noa/cache/packagelist.tar.xz")?);
    let mut buffer = [0; 8192];

    // ファイルのハッシュ値を計算
    while let Ok(n) = file.read(&mut buffer) {
        if n == 0 {
            break;
        }
        hasher.update(&buffer[..n]);
    }
    let computed_hash = hasher.finalize().to_hex();

    // 事前保存したハッシュ値を取得
    let stored_hash = fs::read_to_string("/etc/noa/cache/packagelist.blake3")?
        .trim()
        .to_string();

    // 照合
    if *computed_hash == *stored_hash {
        return Ok(());
    } else {
        err!("ERROR: BLAKE3 hash values do not match. File may have been tampered");
        err!("program aborted.");
        std::process::exit(1);
    }
}
