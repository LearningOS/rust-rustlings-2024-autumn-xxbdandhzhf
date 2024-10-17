//! This is the build script for both tests7 and tests8.
//!
//! You should modify this file to make both exercises pass.

use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    // 获取当前时间戳
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs();

    // 设置环境变量 `TEST_FOO`，其值为当前的 UNIX 时间戳
    println!("cargo:rustc-env=TEST_FOO={}", timestamp);

    // 如果需要在 tests8 中启用 "pass" 特性，可以在这里设置
    // 例如，启用 "pass" 特性
    println!("cargo:rustc-cfg=feature=\"pass\"");
}
