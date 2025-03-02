use crate::consts::{bold, red, reset};
use crate::{err, paths::PACKAGE_LIST_DIR};
use std::{fmt::Display, path::Path};

pub fn search_package<T: AsRef<str> + Display>(package: T) {
    let path = Path::new(PACKAGE_LIST_DIR);

    let mut dir = path.read_dir().unwrap_or_else(|e| {
        err!(
            "{red}{bold}!!!!{reset}{red} Failed to read {}: {} {red}{bold}!!!!{reset}",
            PACKAGE_LIST_DIR,
            e
        );
        std::process::exit(1);
    });

    let _ = dir
        .find(|f| {
            let f = f.as_ref().unwrap_or_else(|e| {
                err!("Error find at noa-inst/src/search.rs:20:84: {}", e);
                std::process::exit(1);
            });
            let temp = f.path();
            let file_name = temp.file_name().unwrap().to_str().unwrap();

            file_name == package.as_ref()
        })
        .unwrap_or_else(|| {
            err!("{}: Package not found", package);
            std::process::exit(1);
        });
}
