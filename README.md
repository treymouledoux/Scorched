# Scorched

A simple logging library for scorching all those pesky bugs.

> [!NOTE]
> The current minimum supported Rust version is: 1.70.0 (Last checked on 8/31/2026)

## Example

This example shows how to use `log_this` to log a message and check optional values.

```rust
use scorched::*;

let something = Some(5);
let nothing = None::<i32>;

something.log_except(LogImportance::Error, "This should not be logged");
nothing.log_except(LogImportance::Error, "This should be logged");

log_this(LogData {
    LogImportance::Debug,
    "All of the tests have now finished!"
});
```

You can also use the `log_except` method on `Option` to log a message if the value is `None`.

```rust
use scorched::*;

let bad_value = None::<i32>;

bad_value.log_except(LogImportance::Error, "This should be logged");
```

> [!TIP]
> If you like you can use the `logf!` macro to log a message and format a string without explicitly needing to run the format macro.

```rust
let thread_id = "7"
logf!(Info, "Heartbeat from thread {}.", thread_id);
```