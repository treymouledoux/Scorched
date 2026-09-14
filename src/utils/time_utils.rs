use chrono::{DateTime, Utc};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

/// Specifies the time format used for the logs: support DT, D, and T
pub(crate) enum TimeFormat {
    DateTime,
    Date,
    Time,
}

/// Gets the current time in the specified format (DateTime, Date, or Time)
pub(crate) fn get_formatted_time(time_format: TimeFormat) -> String {
    let since_the_epoch = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_else(|e| {
            eprintln!("[logging] system clock is before UNIX_EPOCH: {e}");
            Duration::default()
        });

    let time = DateTime::<Utc>::from_timestamp(since_the_epoch.as_secs() as i64, 0)
        .unwrap_or_else(|| DateTime::<Utc>::from_timestamp(0, 0).expect("epoch is always valid"))
        .to_rfc3339();

    let (date, rest) = time.split_once('T').unwrap_or((time.as_str(), ""));
    let time_only = rest.split_once('+').map(|(t, _)| t).unwrap_or(rest);

    match time_format {
        TimeFormat::DateTime => format!("{date} {time_only}"),
        TimeFormat::Date => date.to_string(),
        TimeFormat::Time => time_only.to_string(),
    }
}

pub(crate) fn format_log_line(tag: &str, message: &str, timestamp: &str) -> String {
    format!("{timestamp} [{tag}] {message}\n")
}
