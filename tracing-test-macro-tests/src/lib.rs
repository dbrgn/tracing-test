//! This crate is only here to test the `tracing-test-macro` crate (because proc macros cannot be
//! tested from within the crate itself).

#[cfg(test)]
mod tests {
    use tracing::{info, warn};
    use tracing_test::traced_test;

    #[tokio::test]
    #[traced_test(targeted_crate)]
    async fn test_logs_are_captured() {
        // Local log
        info!("This is being logged on the info level");

        info!("CountMe");
        // Log from a spawned task (which runs in a separate thread)
        tokio::spawn(async {
            warn!("This is being logged on the warn level from a spawned task");
            info!("CountMe");
        })
        .await
        .unwrap();

        // Ensure that `logs_contain` works as intended
        assert!(logs_contain("logged on the info level"));
        assert!(logs_contain("logged on the warn level"));
        assert!(!logs_contain("logged on the error level"));

        // Ensure that `logs_assert` works as intended (with a closure)
        logs_assert(|lines: &[&str]| {
            match lines.iter().filter(|line| line.contains("CountMe")).count() {
                2 => Ok(()),
                n => Err(format!("Count should be 2, but was {}", n)),
            }
        });

        // Ensure that `logs_assert` works as intended (with a function)
        fn assert_fn(lines: &[&str]) -> Result<(), String> {
            match lines.iter().filter(|line| line.contains("CountMe")).count() {
                2 => Ok(()),
                n => Err(format!("Count should be 2, but was {}", n)),
            }
        }
        logs_assert(assert_fn);
    }

    #[traced_test]
    #[test]
    fn annotate_sync_test() {
        assert!(!logs_contain("Logging from a non-async test"));
        info!("Logging from a non-async test");
        assert!(logs_contain("Logging from a non-async test"));
        assert!(!logs_contain("This was never logged"));
    }

    #[traced_test]
    #[test]
    fn no_log_from_other_test1() {
        info!("log count");
        logs_assert(|lines: &[&str]| {
            match lines
                .iter()
                .filter(|line| line.contains("log count"))
                .count()
            {
                1 => Ok(()),
                n => Err(format!("Count should be 1, but was {}", n)),
            }
        });
    }

    #[traced_test]
    #[test]
    fn no_log_from_other_test2() {
        info!("log count");
        logs_assert(|lines: &[&str]| {
            match lines
                .iter()
                .filter(|line| line.contains("log count"))
                .count()
            {
                1 => Ok(()),
                n => Err(format!("Count should be 1, but was {}", n)),
            }
        });
    }
}
