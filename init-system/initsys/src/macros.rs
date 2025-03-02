#[macro_export]
macro_rules! info{
    ($($msg: expr), *) => {
        let fmt = format!( $( $msg ), *);
        println!("[\x1b[32;1m*\x1b[0m] {}", fmt);
    }
}

#[macro_export]
macro_rules! wait{
    ($($msg: expr), *) => {
        let fmt = format!( $( $msg ), *);
        println!("[\x1b[33;1m*\x1b[0m] {}", fmt);
    }
}

#[macro_export]
macro_rules! failed {
    ($($msg: expr), *) => {
        let fmt = format!( $( $msg ), *);
        println!("[\x1b[31;1m*\x1b[0m] {}", fmt);
    }
}

#[macro_export]
macro_rules! write_to_log {
    ($service_name: expr, $msg: expr) => {{
        use crate::failed;
        use std::fs;
        use std::io::Write;
        use std::path;

        let pt = path::PathBuf::new()
            .join("/")
            .join("etc")
            .join("noa")
            .join("logs")
            .join("service");

        let path = pt.join(format!("{}.log", $service_name));

        if !path.exists() {
            fs::create_dir_all(pt).unwrap_or_else(|e| {
                failed!("Failed to create log directory!: {e}");
            });
        }

        let mut file = fs::File::create(path).unwrap_or_else(|e| {
            failed!("\x1b[31;1m!!!! Failed to open log file !!!!\x1b[0m");
            failed!("{e}");
            std::process::exit(1);
        });

        writeln!(file, "{}", $msg).unwrap_or_else(|e| {
            failed!("Failed to write log file: {e}");
        });
    }};
}
