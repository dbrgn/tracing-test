# tracing-test

This crate provides an easy way to enable logging in tests that use
[tracing](https://tracing.rs/), even if they're async. Additionally, it adds a
way to assert that certain things were logged.


## Example

First, add a dependency on `tracing-test` in `Cargo.toml`:

```toml
tokio = { version = "0.2", features = ["rt-threaded", "macros"] }
tracing = "0.1"
tracing-test = "0.1"
```

Then, annotate your test function with the `#[traced_test]` macro.

```rust
use tracing::{info, warn};
use tracing_test::traced_test;

#[tokio::test]
#[traced_test]
async fn test_logs_are_captured() {
    // Local log
    info!("This is being logged on the info level");

    // Log from a spawned task (which runs in a separate thread)
    tokio::spawn(async {
        warn!("This is being logged on the warn level from a spawned task");
    })
    .await
    .unwrap();

    // Ensure that `logs_contain` works as intended
    assert!(logs_contain("logged on the info level"));
    assert!(logs_contain("logged on the warn level"));
    assert!(!logs_contain("logged on the error level"));
}
```

Done! You can write assertions using the injected `logs_contain` function. Logs
are written to stdout, so they are captured by the cargo test runner by
default, but printed if the test fails.


## Rationale / Why You Need This

Tracing allows you to set a default subscriber within a scope:

```rust
let response = tracing::dispatcher::with_default(&subscriber, || get_response(req));
```

This works fine, as long as no threads are involved. As soon as you use a
multi-threaded test runtime (e.g. the `#[tokio::test]` with the `rt-threaded`
feature) and spawn tasks, the tracing logs in those tasks will not be captured
by the subscriber.

The macro provided in this crate registers a global default subscriber instead.
This subscriber contains a mock writer which logs into a global static buffer.

At the beginning of every test, the macro injects span opening code. The span
uses the name of the test function (unless it's already taken, then a counter
is appended). This means that the logs from a test are prefixed with the test
name, which helps when debugging.

Finally, a function called `logs_contain(value: &str)` is injected into every
annotated test. It filters the logs in the buffer to include only lines
containing ` {span_name}: ` and then searches the value in the matching log
lines.


## License

Copyright © 2020 Threema GmbH, Danilo Bargen and Contributors.

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT) at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
