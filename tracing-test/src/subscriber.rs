use std::{
    io,
    sync::{Mutex, MutexGuard},
};

use tracing_core::Dispatch;
use tracing_subscriber::{fmt::MakeWriter, layer::SubscriberExt, EnvFilter, Layer, Registry};

/// A fake writer that writes into a buffer (behind a mutex).
#[derive(Debug)]
pub struct MockWriter<'a> {
    buf: &'a Mutex<Vec<u8>>,
}

impl<'a> MockWriter<'a> {
    /// Create a new `MockWriter` that writes into the specified buffer (behind a mutex).
    pub fn new(buf: &'a Mutex<Vec<u8>>) -> Self {
        Self { buf }
    }

    /// Give access to the internal buffer (behind a `MutexGuard`).
    fn buf(&self) -> io::Result<MutexGuard<'a, Vec<u8>>> {
        // Note: The `lock` will block. This would be a problem in production code,
        // but is fine in tests.
        self.buf
            .lock()
            .map_err(|_| io::Error::from(io::ErrorKind::Other))
    }
}

impl<'a> io::Write for MockWriter<'a> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        // Lock target buffer
        let mut target = self.buf()?;

        // Print to output if no-log-printing is off and pretty-log-printing is off as well.
        #[cfg(not(any(feature = "no-log-printing", feature = "pretty-log-printing")))]
        print!("{}", String::from_utf8(buf.to_vec()).unwrap());

        // Write to buffer
        target.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.buf()?.flush()
    }
}

impl<'a> MakeWriter<'_> for MockWriter<'a> {
    type Writer = Self;

    fn make_writer(&self) -> Self::Writer {
        MockWriter::new(self.buf)
    }
}

/// Return a new subscriber that writes to the specified [`MockWriter`] and
/// to standard output.
///
/// [`MockWriter`]: struct.MockWriter.html
pub fn get_subscriber(mock_writer: MockWriter<'static>, env_filter: &str) -> Dispatch {
    let filter = EnvFilter::new(env_filter);
    let mock_writer_layer = tracing_subscriber::fmt::layer()
        .with_writer(mock_writer)
        .with_level(true)
        .with_ansi(false)
        .with_filter(filter);

    #[cfg(not(feature = "pretty-log-printing"))]
    let subscriber = Registry::default().with(mock_writer_layer);

    #[cfg(feature = "pretty-log-printing")]
    let subscriber = {
        let print_filter = std::env::var("RUST_LOG").unwrap_or_else(|_| env_filter.to_string());
        let print_filter = EnvFilter::new(print_filter);
        let print_layer = tracing_subscriber::fmt::layer()
            .with_writer(|| TestWriter)
            .event_format(tracing_subscriber::fmt::format().with_line_number(true))
            .with_level(true)
            .with_filter(print_filter);
        Registry::default()
            .with(mock_writer_layer)
            .with(print_layer)
    };

    subscriber.into()
}

/// A tracing writer that interacts well with test output capture.
///
/// Using this writer will make sure that the output is captured normally and only printed
/// when the test fails.
#[cfg(feature = "pretty-log-printing")]
#[derive(Debug)]
struct TestWriter;

#[cfg(feature = "pretty-log-printing")]
impl std::io::Write for TestWriter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        print!(
            "{}",
            std::str::from_utf8(buf).expect("tried to log invalid UTF-8")
        );
        Ok(buf.len())
    }
    fn flush(&mut self) -> std::io::Result<()> {
        std::io::stdout().flush()
    }
}
