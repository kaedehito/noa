#[macro_export]
macro_rules! err {
    ($($msg: expr), *) => {
        let fmt = format!($($msg), *);
        println!("[\x1b[31;1m*\x1b[0m] {fmt}");
    };
}

#[macro_export]
macro_rules! warn {
    ($($msg: expr), *) => {
        let fmt = format!($($msg), *);
        println!("[\x1b[33;1m*\x1b[0m] {fmt}");
    };
}

#[macro_export]
macro_rules! info {
    ($($msg: expr), *) => {
        let fmt = format!($($msg), *);
        println!("[\x1b[32;1m*\x1b[0m] {fmt}");
    };
}
