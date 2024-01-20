mod logger;

mod mobile;

use std::sync::Once;
use lazy_static::lazy_static;

pub use log::*;

lazy_static! {
    // 配置日志
    pub static ref LOG_ONCE: Once = Once::new();
}

pub fn enable(level: i32) {
    LOG_ONCE.call_once(move || {
        logger::setup(level).unwrap();
    });
}

#[test]
fn test_logger() {
    enable(3);
    log::info!("abc");
}
