use chrono::{TimeZone, Utc};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::{LogExpect, LogImportance};

/// Specifies the time format used for the logs support DT, D, and T
pub(crate) enum TimeFormat {
    DateTime,
    Date,
    Time,
}

/// Gets the current time in the specified format (DateTime, Date, or Time)
pub(crate) fn get_formatted_time(time_format: TimeFormat) -> String {
    let now = SystemTime::now();
    let since_the_epoch = now
        .duration_since(UNIX_EPOCH)
        .log_expect(LogImportance::Error, "Failed to get time since the epoch");

    let time = Utc
        .timestamp_opt(since_the_epoch.as_secs() as i64, 0)
        .unwrap()
        .with_timezone(&Utc)
        .to_rfc3339();

    // Splits the time into the needed format
    match time_format {
        TimeFormat::DateTime => time.split('+').collect::<Vec<&str>>()[0]
            .to_string()
            .replace('T', " "),
        TimeFormat::Date => time.split('+').collect::<Vec<&str>>()[0].split('T')
            .collect::<Vec<&str>>()[0]
            .to_string(),
        TimeFormat::Time => time.split('T').collect::<Vec<&str>>()[1].split('+')
            .collect::<Vec<&str>>()[0]
            .to_string(),
    }
}
