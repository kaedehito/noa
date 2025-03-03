use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use tar::Archive;
use xz2::read::XzDecoder;

use crate::err;

pub fn uncompress() -> Result<(), Box<dyn std::error::Error>> {
    let path: &Path = "/etc/noa/cache/packagelist.tar.xz".as_ref();

    if !path.exists() {
        err!(
            "\x1b[31m!!!! ERROR !!!!\x1b[0m {} is not found!",
            path.display()
        );
    }
    let file = File::open("/etc/noa/cache/packagelist.tar.xz")?;
    let buf_reader = BufReader::new(file);
    let xz_decoder = XzDecoder::new(buf_reader);
    let mut archive = Archive::new(xz_decoder);

    let dest = Path::new("/etc/noa/package/package-list/");

    for entry in archive.entries()? {
        let mut entry = entry?;
        let path = entry.path()?;
        println!("{}", path.display());

        entry.unpack(dest)?;
    }

    Ok(())
}
