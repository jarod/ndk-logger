#![allow(non_camel_case_types)]

use std::os::raw::{c_char, c_int};

#[repr(C)]
pub enum android_LogPriority {
  ANDROID_LOG_UNKNOWN = 0,
  ANDROID_LOG_DEFAULT, /* only for SetMinPriority() */
  ANDROID_LOG_VERBOSE,
  ANDROID_LOG_DEBUG,
  ANDROID_LOG_INFO,
  ANDROID_LOG_WARN,
  ANDROID_LOG_ERROR,
  ANDROID_LOG_FATAL,
  ANDROID_LOG_SILENT, /* only for SetMinPriority(); must be last */
}

extern "C" {
  /// Send a simple string to the log.
  pub fn __android_log_write(prio: c_int, tag: *const c_char, text: *const c_char) -> c_int;
}
