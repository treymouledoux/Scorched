use chrono::{DateTime, Utc};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

/// Gets one UTC timestamp for the entire log record.
pub(crate) fn now() -> DateTime<Utc> {
    let since_the_epoch = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_else(|e| {
            eprintln!("[logging] system clock is before UNIX_EPOCH: {e}");
            Duration::default()
        });

    DateTime::<Utc>::from_timestamp(since_the_epoch.as_secs() as i64, 0)
        .unwrap_or_else(|| DateTime::<Utc>::from_timestamp(0, 0).expect("epoch is always valid"))
}

pub(crate) fn format_log_line(tag: &str, message: &str, timestamp: &str) -> String {
    format!("{timestamp} [{tag}] {message}\n")
}
