use crate::{Size, clock};
use std::marker::PhantomData;
use std::time::Duration;

pub trait Rattle: Copy + Default {
    const SIZE: Size;
    const INTERVAL: Duration;
    const FRAMES: &'static [&'static [&'static str]];
}

#[derive(Debug, Clone)]
pub struct Rattler<T: Rattle, const REVERSED: bool = false> {
    interval: Duration,
    interval_ns: u128,
    offset: usize,
    _template: PhantomData<T>,
}

impl<T: Rattle, const REVERSED: bool> Rattler<T, REVERSED> {
    pub fn new() -> Self {
        let interval = T::INTERVAL;
        Self {
            interval,
            interval_ns: interval.as_nanos().max(1),
            offset: 0,
            _template: PhantomData,
        }
    }

    pub fn with_offset(offset: usize) -> Self {
        Self {
            interval: T::INTERVAL,
            interval_ns: T::INTERVAL.as_nanos().max(1),
            offset: if T::FRAMES.is_empty() {
                0
            } else {
                offset % T::FRAMES.len()
            },
            _template: PhantomData,
        }
    }

    pub const fn size(&self) -> Size {
        T::SIZE
    }

    pub fn len(&self) -> usize {
        T::FRAMES.len()
    }

    pub fn is_empty(&self) -> bool {
        T::FRAMES.is_empty()
    }

    fn index_at_elapsed(&self, elapsed: Duration) -> usize {
        let len = T::FRAMES.len();
        let base = base_index_at_elapsed_ns(elapsed.as_nanos(), self.interval_ns, len);
        let shifted = if len == 0 {
            0
        } else {
            (base + self.offset) % len
        };
        playback_index_const::<REVERSED>(shifted, len)
    }

    pub fn rows_at_elapsed(&self, elapsed: Duration) -> &'static [&'static str] {
        let idx = self.index_at_elapsed(elapsed);
        &T::FRAMES[idx]
    }

    pub fn current_rows(&self) -> &'static [&'static str] {
        self.rows_at_elapsed(clock::elapsed())
    }

    pub fn current_row(&self) -> &'static str {
        self.current_rows()[0]
    }

    pub fn index(&self) -> usize {
        self.index_at_elapsed(clock::elapsed())
    }

    pub const fn is_reversed(&self) -> bool {
        REVERSED
    }

    pub const fn interval(&self) -> Duration {
        self.interval
    }

    pub fn set_interval(mut self, interval: Duration) -> Self {
        self.interval = interval;
        self.interval_ns = interval.as_nanos().max(1);
        self
    }

    pub fn offset(mut self, offset: usize) -> Self {
        self.offset = if T::FRAMES.is_empty() {
            0
        } else {
            offset % T::FRAMES.len()
        };
        self
    }
}

impl<T: Rattle, const REVERSED: bool> Default for Rattler<T, REVERSED> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Rattle> Rattler<T, false> {
    pub fn reverse(self) -> Rattler<T, true> {
        Rattler {
            interval: self.interval,
            interval_ns: self.interval_ns,
            offset: self.offset,
            _template: PhantomData,
        }
    }
}

impl<T: Rattle> Rattler<T, true> {
    pub fn reverse(self) -> Rattler<T, false> {
        Rattler {
            interval: self.interval,
            interval_ns: self.interval_ns,
            offset: self.offset,
            _template: PhantomData,
        }
    }
}

impl<T: Rattle, const REVERSED: bool> Iterator for Rattler<T, REVERSED> {
    type Item = &'static [&'static str];

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.current_rows())
    }
}

fn playback_index_const<const REVERSED: bool>(index: usize, len: usize) -> usize {
    if len == 0 {
        return 0;
    }

    if REVERSED { len - 1 - index } else { index }
}

fn base_index_at_elapsed_ns(elapsed_ns: u128, interval_ns: u128, len: usize) -> usize {
    if len == 0 {
        return 0;
    }

    ((elapsed_ns / interval_ns.max(1)) as usize) % len
}
