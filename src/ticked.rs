use crate::util::apply_direction;
use crate::{Rattle, Rattler, Size};
use core::marker::PhantomData;
use core::time::Duration;

#[derive(Debug, Clone)]
pub struct TickedRattler<T: Rattle, const REVERSED: bool = false> {
    pub(crate) interval_ms: u64,
    pub(crate) offset: usize,
    pub(crate) current: usize,
    pub(crate) _marker: PhantomData<T>,
}

impl<T: Rattle, const REVERSED: bool> TickedRattler<T, REVERSED> {
    pub fn new() -> Self {
        Self {
            interval_ms: T::INTERVAL.as_millis().max(1) as u64,
            offset: 0,
            current: 0,
            _marker: PhantomData,
        }
    }

    pub fn with_offset(offset: usize) -> Self {
        Self {
            interval_ms: T::INTERVAL.as_millis().max(1) as u64,
            offset: if T::FRAMES.is_empty() {
                0
            } else {
                offset % T::FRAMES.len()
            },
            current: 0,
            _marker: PhantomData,
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

    pub fn current_frames(&self) -> &'static [&'static str] {
        let len = T::FRAMES.len();
        if len == 0 {
            return &[];
        }

        let shifted = (self.current + self.offset) % len;
        let idx = apply_direction::<REVERSED>(shifted, len);
        &T::FRAMES[idx]
    }

    pub fn current_frame(&self) -> &'static str {
        self.current_frames()[0]
    }

    pub fn index(&self) -> usize {
        let len = T::FRAMES.len();
        if len == 0 {
            return 0;
        }

        apply_direction::<REVERSED>((self.current + self.offset) % len, len)
    }

    pub fn tick(&mut self) -> &'static [&'static str] {
        let len = T::FRAMES.len();
        if len == 0 {
            return &[];
        }

        self.current = (self.current + 1) % len;
        self.current_frames()
    }

    pub fn tick_by(&mut self, steps: usize) -> &'static [&'static str] {
        let len = T::FRAMES.len();
        if len == 0 {
            return &[];
        }

        self.current = (self.current + (steps % len)) % len;
        self.current_frames()
    }

    pub fn reset(&mut self) {
        self.current = 0;
    }

    pub const fn is_reversed(&self) -> bool {
        REVERSED
    }

    pub const fn interval(&self) -> Duration {
        Duration::from_millis(self.interval_ms)
    }

    pub fn set_interval(mut self, interval: Duration) -> Self {
        self.interval_ms = interval.as_millis().max(1) as u64;
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

    pub fn into_timed(self) -> Rattler<T, REVERSED> {
        Rattler {
            interval_ms: self.interval_ms,
            offset: self.offset,
            _marker: PhantomData,
        }
    }
}

impl<T: Rattle, const REVERSED: bool> Default for TickedRattler<T, REVERSED> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Rattle> TickedRattler<T, false> {
    pub fn reverse(self) -> TickedRattler<T, true> {
        TickedRattler {
            interval_ms: self.interval_ms,
            offset: self.offset,
            current: self.current,
            _marker: PhantomData,
        }
    }
}

impl<T: Rattle> TickedRattler<T, true> {
    pub fn reverse(self) -> TickedRattler<T, false> {
        TickedRattler {
            interval_ms: self.interval_ms,
            offset: self.offset,
            current: self.current,
            _marker: PhantomData,
        }
    }
}

impl<T: Rattle, const REVERSED: bool> Iterator for TickedRattler<T, REVERSED> {
    type Item = &'static [&'static str];

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.tick())
    }
}
