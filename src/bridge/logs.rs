//! Logging functions exposed to C++ to initialize and unify logs from both sides.
//!
//! There's an initialization function that must be called in `main.cpp` to tell
//! the plugin where to log. The other functions are for C++ to use to share a log
//! file with Rust. For now, C++ must pass a preformatted-string to these functions.
//! This is wasteful, but exposing Rust macros to C++ is not possible.

use std::ffi::OsString;
use std::fs::File;
#[cfg(target_os = "windows")]
use std::os::windows::ffi::OsStringExt;
use std::path::Path;

use simplelog::*;

/// Create a log file in the directory SKSE wants us to, and initialize a logger.
pub fn initialize_logging(_logdir: &cxx::CxxVector<u16>) {
    #[cfg(not(target_os = "windows"))]
    let chonky_path = OsString::from("placeholder");
    #[cfg(target_os = "windows")]
    let chonky_path = OsString::from_wide(_logdir.as_slice());
    let path = Path::new(chonky_path.as_os_str()).with_file_name("skse-rust-template.log");

    let Ok(logfile) = File::create(path) else {
        // Welp, we failed and I have nowhere to write the darn error. Ha ha.
        return;
    };
    // You might look this up from config or user settings.
    let log_level = log::LevelFilter::Debug;
    let config = simplelog::ConfigBuilder::new()
        .set_thread_level(LevelFilter::Off)
        .set_level_padding(simplelog::LevelPadding::Right)
        .set_location_level(LevelFilter::Trace)
        .set_target_level(LevelFilter::Trace)
        .build();
    let Ok(_) = WriteLogger::init(log_level, config, logfile) else {
        // oh dear
        return;
    };
    log::info!(
        "skse-rust-template version {} coming online.",
        env!("CARGO_PKG_VERSION")
    );
}

/// For C++, log at the error level.
pub fn log_error(message: String) {
    log::error!("{}", message);
}

/// For C++, log at the warn level.
pub fn log_warn(message: String) {
    log::warn!("{}", message);
}

/// For C++, log at the info level.
pub fn log_info(message: String) {
    log::info!("{}", message);
}

/// For C++, log at the debug level.
pub fn log_debug(message: String) {
    log::debug!("{}", message);
}

/// For C++, log at the trace level.
pub fn log_trace(message: String) {
    log::trace!("{}", message);
}
