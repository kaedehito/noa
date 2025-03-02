use std::io::{self, Write};

mod consts;
mod macros;
mod paths;
mod search;
use consts::{green, red, reset};

macro_rules! sleep {
    ($second: expr) => {{
        use std::process::Command;
        Command::new("sleep").arg($second).status().unwrap();
    }};
}

pub fn install(packages: &[String], ask: bool, _build: bool) {
    info!("Searching packages...");
    // search package(s)
    packages.iter().for_each(search::search_package);

    info!("All packages: {}", packages.join(", "));

    if ask {
        info!("Process to install package(s)?");
        print!("[{green}y{reset}/{red}n{reset}]{reset} ");
        io::stdout().flush().unwrap();
        let mut buf = String::new();
        io::stdin().read_line(&mut buf).unwrap();
        let p = matches!(buf.trim(), "y" | "yes" | "");
        if !p {
            err!("Installation canceled");
            return;
        }
    }

    let total = packages.len();

    for (current, package) in packages.iter().enumerate() {
        let current = current + 1;
        info!("Downloading {}.tar.xz ({}/{})", package, current, total);
        sleep!("2");
        info!("Downloading {}-build.sh ({}/{})", package, current, total);
        sleep!("0.6");
        // TODO
        warn!("Building {} ({}/{})\n", package, current, total);
        sleep!("6");
        info!("Build end");
        // TODO: Building logic
        warn!("Installing {} ({}/{})", package, current, total);
        sleep!("4");
        info!("{} is installed!", package);
    }
    info!("All package installed. cleaning...");
    sleep!("2");
    info!("Success!");
}
