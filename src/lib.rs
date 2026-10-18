#![doc = include_str!("../README.md")]

mod macros;
mod utils;

#[cfg(test)]
mod test;

use std::{
    fmt::Debug,
    fs::OpenOptions,
    io::prelude::*,
    path::Path,
    sync::{OnceLock, RwLock},
};

use utils::{importance_tags::*, time_utils};

/// A message's severity, from most to least important: Error, Warning, Info, Debug.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LogImportance {
    /// An operation failed.
    Error,
    /// An unexpected condition that may need attention.
    Warning,
    /// General application information.
    Info,
    /// Detailed diagnostic information.
    Debug,
}

impl LogImportance {
    fn priority(&self) -> u8 {
        match self {
            LogImportance::Error => 3,
            LogImportance::Warning => 2,
            LogImportance::Info => 1,
            LogImportance::Debug => 0,
        }
    }
}

/// A message and its severity, passed to [`log_this`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LogData {
    /// The message's severity.
    pub importance: LogImportance,
    /// The text to log.
    pub message: String,
}

/// Scorched version, has no internal use
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// The log path, defaults to "logs/"
pub static LOG_PATH: OnceLock<&str> = OnceLock::new();
/// The log prefix, defaults to ""
pub static LOG_PREFIX: OnceLock<&str> = OnceLock::new();
// Debug allows every level, including before the first setter call.
static MIN_LOG_LEVEL: RwLock<LogImportance> = RwLock::new(LogImportance::Debug);

/// Sets the process-wide log directory. The default is `logs/`.
///
/// Call this once during startup, before logging. A trailing separator is added
/// when needed. This does not change an environment variable or create the directory.
///
/// # Panics
///
/// Panics if the log directory has already been configured.
pub fn set_logging_path(path: &str) {
    let path = if path.ends_with('/') || path.ends_with('\\') {
        path.to_string()
    } else {
        let mut path = path.to_string();
        path.push('/');
        path
    };

    match LOG_PATH.set(Box::leak(path.into_boxed_str())) {
        Ok(_) => (),
        Err(_) => panic!("Failed to set logging path"),
    }
}

/// Sets the process-wide text prepended to each message, followed by a space.
///
/// Call this once during startup. By default, messages have no prefix.
///
/// # Panics
///
/// Panics if the prefix has already been configured.
pub fn set_log_prefix(prefix: String) {
    match LOG_PREFIX.set(Box::leak(prefix.into_boxed_str())) {
        Ok(_) => (),
        Err(_) => panic!("Failed to set logging prefix"),
    }
}

/// Sets the minimum severity for both console and file output.
///
/// Messages at or above `level` are logged. The default is [`LogImportance::Debug`],
/// which allows every level. This function can be called repeatedly, including
/// from multiple threads. Setting `Debug` restores the default filtering behavior.
/// A log call already in progress may finish using the previous threshold.
pub fn set_min_log_level(level: LogImportance) {
    let mut min_level = MIN_LOG_LEVEL.write().unwrap_or_else(|err| err.into_inner());
    *min_level = level;
}

/// Returns whether `level` meets the current minimum severity.
///
/// Use this before constructing expensive messages. [`logf!`] checks this
/// automatically before evaluating its formatting arguments. The threshold can
/// change between this check and a subsequent log call.
pub fn enabled(level: LogImportance) -> bool {
    let min_level = MIN_LOG_LEVEL.read().unwrap_or_else(|err| err.into_inner());
    level.priority() >= min_level.priority()
}

/// Appends a message to a daily UTC log file and prints it to the console.
///
/// Filtered messages produce no output and do not create directories or files.
/// File errors are reported to stderr. Directory or file-open failures skip
/// console output; a file-write failure still allows console output.
pub fn log_this(data: LogData) {
    if !enabled(data.importance) {
        return;
    }

    // One instant keeps the filename and both timestamps consistent at midnight.
    let timestamp = time_utils::now();
    let log_path = LOG_PATH.get().unwrap_or(&"logs/");

    if !std::path::Path::new(log_path).exists() {
        if let Err(e) = std::fs::create_dir_all(log_path) {
            eprintln!("[logging] failed to create log dir {log_path:?}: {e}");
            return;
        }
    }

    let filename = Path::new(log_path).join(format!("{}.log", timestamp.format("%Y-%m-%d")));
    let mut file = match OpenOptions::new().append(true).create(true).open(filename) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("[logging] failed to open log file: {e}");
            return;
        }
    };

    // Appends the prefix to the message if it exists
    let message = {
        match LOG_PREFIX.get() {
            Some(val) => format!("{} {}", val, data.message),
            None => data.message,
        }
    };

    let tag = match data.importance {
        LogImportance::Error => (ERROR_TAG, "ERROR"),
        LogImportance::Warning => (WARNING_TAG, "WARNING"),
        LogImportance::Info => (INFO_TAG, "INFO"),
        LogImportance::Debug => (DEBUG_TAG, "DEBUG"),
    };

    let line = time_utils::format_log_line(
        tag.1,
        &message,
        &timestamp.format("%Y-%m-%d %H:%M:%S").to_string(),
    );
    if let Err(e) = file.write_all(line.as_bytes()) {
        eprintln!("[logging] failed to write to log file: {e}");
    }

    println!("{} {} {}", timestamp.format("%H:%M:%S"), tag.0, message);
}

/// Unwraps a successful value, or logs a message and panics on failure.
pub trait LogExpect<T> {
    /// Returns the contained value from `Some` or `Ok`.
    ///
    /// # Panics
    ///
    /// On `None` or `Err`, logs `msg` at `importance` and then panics. Filtering
    /// suppresses only the log output, never the panic. For `Err`, the panic also
    /// includes the error's debug representation.
    #[track_caller]
    fn log_expect(self, importance: LogImportance, msg: &str) -> T;
}

impl<T, E: Debug> LogExpect<T> for Result<T, E> {
    #[track_caller]
    fn log_expect(self, importance: LogImportance, msg: &str) -> T {
        match self {
            Ok(val) => val,
            Err(err) => {
                log_this(LogData {
                    importance,
                    message: msg.to_string(),
                });

                panic!("{}: {:?}", msg, err);
            }
        }
    }
}

impl<T> LogExpect<T> for Option<T> {
    #[track_caller]
    fn log_expect(self, importance: LogImportance, msg: &str) -> T {
        match self {
            Some(val) => val,
            None => {
                log_this(LogData {
                    importance,
                    message: msg.to_string(),
                });

                panic!("{}", msg);
            }
        }
    }
}
