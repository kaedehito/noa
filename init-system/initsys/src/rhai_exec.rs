use rhai::run;

use crate::failed;

#[allow(dead_code)]
pub fn run_rhai(src: &str, _service: &str) {
    run(src).unwrap_or_else(|e| {
        failed!("!!! rhai error !!!");
        failed!("{}", e);
    });
}
