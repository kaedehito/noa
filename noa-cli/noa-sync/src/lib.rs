use download_file::download_file;
use std::fs::{self};
use std::path::Path;

mod download_file;
mod get_mirror;
mod macros;
mod uncompress;

pub fn sync_package_list() {
    let mirrors = get_mirror::get_mirror();

    info!("Downloading packagelist.tar.xz");
    for mirror in mirrors {
        let download_url = format!("{}/noa/packagelist.tar.xz", mirror);
        if let Err(_) = download_file(&download_url, "/etc/noa/cache/packagelist.tar.xz") {
            info!("mirror chenge to: {}", mirror);
            continue;
        } else {
            break;
        }
    }

    let path: &Path = "/etc/noa/package/package-list/".as_ref();
    if path.exists() {
        info!("Removing old package list...");
        fs::remove_dir_all(path).unwrap_or_else(|e| {
            err!("Failed to remove old pacakge list: {e}");
            return;
        });
    }

    if !path.exists() {
        info!("Creating package list directory...");
        fs::create_dir_all(path).unwrap_or_else(|e| {
            err!("Failed to create package list directory: {e}");
        })
    }

    info!("Uncompressing package list");
    uncompress::uncompress().unwrap_or_else(|e| {
        err!("Failed to uncompress package list: {e}");
        return;
    });

    info!("Cleaning...");
    fs::remove_file("/etc/noa/cache/packagelist.tar.xz").unwrap_or_else(|e| {
        err!("Failed to remove packagelist.tar.xz: {e}");
        return;
    });
}
