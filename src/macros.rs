/// Logs a formatted message if its severity is enabled.
///
/// Formatting arguments are evaluated only when the level is enabled. Avoid
/// arguments whose side effects are required by your application.
///
/// ```no_run
/// scorched::logf!(Info, "Heartbeat from thread {}.", 7);
/// ```
#[macro_export]
macro_rules! logf {
    ($importance:ident, $($arg:tt)*) => {{
        let importance = $crate::LogImportance::$importance;
        if $crate::enabled(importance) {
            $crate::log_this($crate::LogData {
                importance,
                message: ::std::format!($($arg)*),
            });
        }
    }};
}
