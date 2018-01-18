extern crate log;

mod ffi;

use std::fmt;
use std::os::raw::{c_char, c_int};
use std::result;
use log::{Level, Log, Metadata, Record};
use ffi::android_LogPriority;

#[derive(Debug)]
pub struct Error;
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ndk_logger.Error")
    }
}
impl std::error::Error for Error {
    fn description(&self) -> &str {
        "ndk_logger.Error"
    }
}

type Result<T> = result::Result<T, Error>;

pub struct Logger;

impl Logger {
    pub fn new() -> Result<Logger> {
        Ok(Logger {})
    }
}

impl Log for Logger {
    fn enabled(&self, _metadata: &Metadata) -> bool {
        true
    }

    fn log(&self, record: &Record) {
        let prio = match record.level() {
            Level::Error => android_LogPriority::ANDROID_LOG_ERROR,
            Level::Warn => android_LogPriority::ANDROID_LOG_WARN,
            Level::Info => android_LogPriority::ANDROID_LOG_INFO,
            Level::Debug => android_LogPriority::ANDROID_LOG_DEBUG,
            Level::Trace => android_LogPriority::ANDROID_LOG_VERBOSE,
        };
        let text = format!("{}", record.args());
        let _ret = unsafe {
            ffi::__android_log_write(
                prio as c_int,
                record.target().as_ptr() as *const c_char,
                text.as_ptr() as *const c_char,
            )
        };
    }

    fn flush(&self) {}
}
