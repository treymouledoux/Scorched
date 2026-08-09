mod macros;
mod utils;

mod test;

use std::{
    fmt::Debug,
    fs::{File, OpenOptions},
    io::prelude::*,
    sync::OnceLock,
};

use utils::{importance_tags::*, time_utils};

/// Is given to the log functions to specify the importance of the log and is provided in the header of the log
pub enum LogImportance {
    Error,
    Warning,
    Info,
    Debug,
}

/// Is given to the log functions to specify the log importance and the message of the log
pub struct LogData {
    pub importance: LogImportance,
    pub message: String,
}

/// Scorched version, has no internal use
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// The log path, defaults to "logs/"
pub static LOG_PATH: OnceLock<&str> = OnceLock::new();
/// The log prefix, defaults to ""
pub static LOG_PREFIX: OnceLock<&str> = OnceLock::new();

/// Changes the environment variable for logging path
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

/// Changes the environment variable for logging prefix, this is the text that is displayed before the log message
pub fn set_log_prefix(prefix: String) {
    match LOG_PREFIX.set(Box::leak(prefix.into_boxed_str())) {
        Ok(_) => (),
        Err(_) => panic!("Failed to set logging prefix"),
    }
}

/// Logs the given data to the console with the error type and then to a file
pub fn log_this(data: LogData) {
    // Creates logs folder if it doesn't exist
    if !std::path::Path::new(LOG_PATH.get().unwrap_or(&"logs/")).exists() {
        std::fs::create_dir_all(LOG_PATH.get().unwrap_or(&"logs/")).log_expect(
            LogImportance::Error,
            "Failed to create full path to logs folder",
        );
    }

    let file = OpenOptions::new().append(true).create(true).open(format!(
        "{}{}.log",
        LOG_PATH.get().unwrap_or(&"logs/"),
        time_utils::get_formatted_time(time_utils::TimeFormat::Date)
    ));

    // Appends the prefix to the message if it exists
    let message = {
        match LOG_PREFIX.get() {
            Some(val) => format!("{} {}", val, data.message),
            None => data.message,
        }
    };

    fn write_log(importance: LogImportance, message: &str, file: &mut File) {
        let tag = match importance {
            LogImportance::Error => (ERROR_TAG, "ERROR"),
            LogImportance::Warning => (WARNING_TAG, "WARNING"),
            LogImportance::Info => (INFO_TAG, "INFO"),
            LogImportance::Debug => (DEBUG_TAG, "DEBUG"),
        };

        file.write_all(
            format!(
                "{} [{}] {}\n",
                time_utils::get_formatted_time(time_utils::TimeFormat::DateTime),
                tag.1,
                message
            )
            .as_bytes(),
        )
        .unwrap();

        println!(
            "{} {} {}",
            time_utils::get_formatted_time(time_utils::TimeFormat::Time),
            tag.0,
            message
        );
    }

    match data.importance {
        LogImportance::Error => write_log(LogImportance::Error, &message, &mut file.unwrap()),
        LogImportance::Warning => write_log(LogImportance::Warning, &message, &mut file.unwrap()),
        LogImportance::Info => write_log(LogImportance::Info, &message, &mut file.unwrap()),
        LogImportance::Debug => write_log(LogImportance::Debug, &message, &mut file.unwrap()),
    }
}

pub trait LogExpect<T, E: Debug> {
    fn log_expect(self, importance: LogImportance, msg: &str) -> T;
}

impl<T, E: Debug> LogExpect<T, E> for Result<T, E> {
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

impl<T> LogExpect<T, ()> for Option<T> {
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
