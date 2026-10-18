# Scorched

A simple logging library for scorching all those pesky bugs.

Scorched writes messages to the console and daily log files, with four severity
levels and a configurable minimum level. Requires Rust **1.70 or newer**.

## Installation

```toml
[dependencies]
scorched = "0.7"
```

## Logging

```rust,no_run
use scorched::{log_this, LogData, LogImportance};

log_this(LogData {
    importance: LogImportance::Info,
    message: "Application started".into(),
});

let thread_id = 7;
scorched::logf!(Info, "Heartbeat from thread {}.", thread_id);
```

Messages are appended to `logs/YYYY-MM-DD.log`. Filenames and timestamps use UTC.
The console shows colored severity tags; files contain plain text.

## Minimum log level

Severity order is `Error > Warning > Info > Debug`. By default, every level is
logged. The minimum applies to both console and file output, including messages
from `log_this`, `logf!`, and `log_expect`.

```rust,no_run
use scorched::{set_min_log_level, LogImportance};

set_min_log_level(LogImportance::Warning);
scorched::logf!(Error, "This is logged");
scorched::logf!(Warning, "This is also logged");
scorched::logf!(Info, "This is suppressed");
scorched::logf!(Debug, "This is suppressed too");

// Change the threshold at any time. Debug allows all levels again.
set_min_log_level(LogImportance::Debug);
```

Suppressed calls do not create directories or files. `logf!` also skips evaluation
of its formatting arguments when the level is disabled, so avoid arguments with
side effects your application needs. For manually constructed messages, check
`scorched::enabled(level)` before doing expensive work.

The threshold is process-wide and can be changed from multiple threads. An
in-progress log call may finish using the previous threshold.

## Directory and prefix

```rust,no_run
scorched::set_logging_path("application-logs");
scorched::set_log_prefix("[my-app]".into());
scorched::logf!(Info, "Ready");
```

Configure the directory and prefix once during startup, before logging. Each
setter panics if called a second time. The directory is created on the first
enabled log call. These settings do not modify environment variables.

## Logging failed expectations

`log_expect` works like `expect`: it returns the value of `Some` or `Ok`, and logs
then panics for `None` or `Err`. **Filtering never suppresses the panic.** The
`Result` panic includes the underlying error's debug representation.

```rust,no_run
use scorched::{LogExpect, LogImportance};

let value = Some(5).log_expect(LogImportance::Error, "Missing value");
assert_eq!(value, 5);

let missing = None::<i32>;
missing.log_expect(LogImportance::Error, "Required value is missing");
```

## Development

```sh
cargo fmt --all -- --check
cargo clippy --locked --all-targets -- -D warnings
cargo test --locked
cargo +1.70.0 test --locked
```

Logging integration tests run in separate executables to isolate global settings.
README examples are compiled as documentation tests.

## License

MIT. See [LICENSE](https://github.com/treymouledoux/Scorched/blob/master/LICENSE).
