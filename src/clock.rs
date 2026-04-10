use std::sync::LazyLock;
use std::time::{Duration, Instant};

static INSTANT: LazyLock<Instant> = LazyLock::new(Instant::now);

pub(crate) fn elapsed() -> Duration {
    INSTANT.elapsed()
}
