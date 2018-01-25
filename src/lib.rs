extern crate log;

mod ffi;

use std::os::raw::{c_char, c_int};
use log::{Level, Log, Metadata, Record, SetLoggerError};
use ffi::android_LogPriority;

pub struct Logger;

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

pub fn init() -> Result<(), SetLoggerError> {
    log::set_boxed_logger(Box::new(Logger {}))
}
