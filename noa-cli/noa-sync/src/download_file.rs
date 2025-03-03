use indicatif::{ProgressBar, ProgressStyle};
use reqwest::blocking;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

use crate::{err, info};

pub fn download_file<P: AsRef<Path>>(url: &str, to: P) -> Result<File, Box<dyn std::error::Error>> {
    let Ok(mut response) = blocking::get(url) else {
        err!("Failed to download file");
        return Err("".into());
    };

    let total_size = response.content_length().unwrap_or_else(|| {
        err!("Failed to get content length: response.content_length() result is None (noa-sync/src/download_file.rs:14:47)");
        std::process::exit(1);
    });

    info!("Size: {} bytes", total_size);

    // -- status bar --
    let pb = ProgressBar::new(total_size);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("[{bar:40.white/white}]  {bytes}/{total_bytes} ({eta})")
            .unwrap()
            .progress_chars("#>-"),
    );

    let mut file = File::create(to).unwrap_or_else(|e| {
        err!("Failed to create packagelist.tar.xz: {}", e);
        std::process::exit(1);
    });

    // -- download file --
    let mut downloaded: u64 = 0;
    let mut buffer = [0; 4096];

    while let Ok(bytes_read) = response.read(&mut buffer) {
        if bytes_read == 0 {
            break;
        }

        file.write_all(&buffer[..bytes_read]).unwrap_or_else(|e| {
            err!("Failed to write to packagelist.tar.xz: {}", e);
            std::process::exit(1);
        });

        downloaded += bytes_read as u64;

        pb.set_position(downloaded);
    }

    pb.finish();
    Ok(file)
}
