mod common;

use scorched::{enabled, log_this, set_min_log_level, LogData, LogExpect, LogImportance};
use std::panic::catch_unwind;

fn emit_macro(level: LogImportance, message: &str) {
    match level {
        LogImportance::Error => scorched::logf!(Error, "{}", message),
        LogImportance::Warning => scorched::logf!(Warning, "{}", message),
        LogImportance::Info => scorched::logf!(Info, "{}", message),
        LogImportance::Debug => scorched::logf!(Debug, "{}", message),
    }
}

fn expect_panic(action: impl FnOnce() + std::panic::UnwindSafe, expected: &str) {
    let payload = catch_unwind(action).expect_err("log_expect must still panic");
    let message = payload
        .downcast_ref::<String>()
        .map(String::as_str)
        .or_else(|| payload.downcast_ref::<&str>().copied())
        .unwrap();
    assert_eq!(message, expected);
}

#[test]
fn thresholds_filter_every_entry_point_and_can_be_reset() {
    use LogImportance::{Debug, Error, Info, Warning};
    let logs = common::Logs::new("filter", scorched::set_logging_path);
    set_min_log_level(Error);

    log_this(LogData {
        importance: Info,
        message: "suppressed-direct".into(),
    });
    let mut evaluations = 0;
    scorched::logf!(Debug, "suppressed-macro-{}", {
        evaluations += 1;
        evaluations
    });
    assert_eq!(
        evaluations, 0,
        "suppressed macro arguments must not be evaluated"
    );
    expect_panic(
        || {
            None::<i32>.log_expect(Info, "suppressed-option");
        },
        "suppressed-option",
    );
    expect_panic(
        || {
            Err::<i32, _>("cause").log_expect(Info, "suppressed-result");
        },
        "suppressed-result: \"cause\"",
    );
    assert!(
        !logs.path.exists(),
        "suppressed calls must not create a log directory"
    );

    // Explicit expectations also cover equality at every threshold.
    let levels = [Error, Warning, Info, Debug];
    let cases = [
        (Error, [true, false, false, false]),
        (Warning, [true, true, false, false]),
        (Info, [true, true, true, false]),
        (Debug, [true, true, true, true]),
    ];
    let mut expectations = Vec::new();
    let mut expected_lines = 0;
    for (threshold_index, (threshold, allowed)) in cases.into_iter().enumerate() {
        set_min_log_level(threshold);
        for (level_index, level) in levels.into_iter().enumerate() {
            let should_log = allowed[level_index];
            assert_eq!(enabled(level), should_log);
            for entry in ["direct", "macro", "option", "result"] {
                let marker = format!("matrix-{threshold_index}-{level_index}-{entry}");
                match entry {
                    "direct" => log_this(LogData {
                        importance: level,
                        message: marker.clone(),
                    }),
                    "macro" => emit_macro(level, &marker),
                    "option" => expect_panic(
                        || {
                            None::<i32>.log_expect(level, &marker);
                        },
                        &marker,
                    ),
                    "result" => expect_panic(
                        || {
                            Err::<i32, _>("cause").log_expect(level, &marker);
                        },
                        &format!("{marker}: \"cause\""),
                    ),
                    _ => unreachable!(),
                }
                expectations.push((marker, should_log));
                if should_log {
                    expected_lines += 1;
                }
            }
        }
    }

    set_min_log_level(Error);
    scorched::logf!(Debug, "before-reset-must-not-log");
    set_min_log_level(Debug);
    scorched::logf!(Debug, "after-reset-must-log");

    let text = logs.read();
    for (marker, should_log) in expectations {
        assert_eq!(
            text.contains(&format!(" {marker}\n")),
            should_log,
            "{marker}"
        );
    }
    assert!(!text.contains("suppressed-"));
    assert!(!text.contains("before-reset-must-not-log"));
    assert!(text.contains("[DEBUG] after-reset-must-log\n"));
    assert_eq!(text.lines().count(), expected_lines + 1);
}
