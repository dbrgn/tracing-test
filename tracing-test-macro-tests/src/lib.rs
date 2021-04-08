//! This crate is only here to test the `tracing-test-macro` crate (because proc macros cannot be
//! tested from within the crate itself).

#[cfg(test)]
mod tests {
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
    fn annotate_target_sync_test() {
        assert!(!logs_contain("Logging from a non-async test"));
        info!(target: "target", "Logging from a non-async but targeted test");
        assert!(logs_contain("Logging from a non-async but targeted test"));
        assert!(!logs_contain("This was never logged"));
    }
}
