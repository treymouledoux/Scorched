mod common;

// No imports from scorched: this catches macro names resolved in the caller.
mod macro_caller {
    pub fn log() {
        scorched::logf!(Info, "macro-{}", 42);
    }
}

#[test]
fn default_logging_preserves_all_levels_prefix_and_success_values() {
    use scorched::{enabled, log_this, set_log_prefix, LogData, LogExpect, LogImportance};
    let logs = common::Logs::new("default", scorched::set_logging_path);
    assert!(scorched::LOG_PATH.get().unwrap().ends_with('/'));
    set_log_prefix("[app]".into());

    for (level, tag) in [
        (LogImportance::Error, "ERROR"),
        (LogImportance::Warning, "WARNING"),
        (LogImportance::Info, "INFO"),
        (LogImportance::Debug, "DEBUG"),
    ] {
        assert!(enabled(level));
        log_this(LogData {
            importance: level,
            message: format!("default-{tag}"),
        });
    }
    macro_caller::log();
    let mut evaluations = 0;
    scorched::logf!(Debug, "evaluations-{}", {
        evaluations += 1;
        evaluations
    });
    assert_eq!(evaluations, 1);

    assert_eq!(
        Some(42).log_expect(LogImportance::Error, "success-must-not-log"),
        42
    );
    assert_eq!(
        Ok::<_, &str>(17).log_expect(LogImportance::Error, "success-must-not-log"),
        17
    );

    let text = logs.read();
    for tag in ["ERROR", "WARNING", "INFO", "DEBUG"] {
        assert!(text.contains(&format!("[{tag}] [app] default-{tag}\n")));
    }
    assert!(text.contains("[INFO] [app] macro-42\n"));
    assert!(text.contains("[DEBUG] [app] evaluations-1\n"));
    assert!(!text.contains("success-must-not-log"));
    assert_eq!(text.lines().count(), 6);
}
