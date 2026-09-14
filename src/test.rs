#[test]
fn formats_line_deterministically() {
    use crate::time_utils::format_log_line;

    assert_eq!(
        format_log_line("ERROR", "boom", "2026-09-14 16:07:03"),
        "2026-09-14 16:07:03 [ERROR] boom\n"
    );
}

/// Sets the log path once, logs every level, and asserts the file
/// actually contains each tagged line. Also covers trailing-slash
/// normalization. This is the only test allowed to call `set_logging_path`.
#[test]
fn writes_all_levels_to_file() {
    use crate::utils::time_utils::{self, TimeFormat};
    use crate::{log_this, set_logging_path, LogData, LogImportance, LOG_PATH};

    let dir = std::env::temp_dir().join(format!("scorched-test-{}", std::process::id()));
    // Passed without a trailing slash on purpose, to exercise normalization.
    set_logging_path(dir.to_str().unwrap());

    for (importance, message) in [
        (LogImportance::Error, "err-marker"),
        (LogImportance::Warning, "warn-marker"),
        (LogImportance::Info, "info-marker"),
        (LogImportance::Debug, "debug-marker"),
    ] {
        log_this(LogData {
            importance,
            message: message.to_string(),
        });
    }

    // Path was normalized to end with a separator.
    assert!(LOG_PATH.get().unwrap().ends_with('/'));

    let log_file = dir.join(format!(
        "{}.log",
        time_utils::get_formatted_time(TimeFormat::Date)
    ));
    let contents = std::fs::read_to_string(&log_file).expect("log file should exist");

    assert!(contents.contains("[ERROR] err-marker"));
    assert!(contents.contains("[WARNING] warn-marker"));
    assert!(contents.contains("[INFO] info-marker"));
    assert!(contents.contains("[DEBUG] debug-marker"));

    let _ = std::fs::remove_dir_all(&dir); // best-effort cleanup
}

/// Smoke test only: verifies the `logf!` macro expands and runs without
/// panicking. It does NOT assert output — file content is covered above.
#[test]
fn logf_macro_runs_without_panicking() {
    use crate::{logf, LogData, LogImportance};

    logf!(Error, "Test error");
    logf!(Warning, "Test warning");
    logf!(Info, "Test info");
    logf!(Debug, "Test debug {}", 42); // also exercise the format path
}
