use crate::util::{apply_direction, frame_at};
use crate::{Rattle, Size, TickedRattler};
use core::marker::PhantomData;
use core::time::Duration;

#[cfg(feature = "std")]
use crate::clock;

#[derive(Debug, Clone)]
pub struct Rattler<T: Rattle, const REVERSED: bool = false> {
    pub(crate) interval_ms: u64,
    pub(crate) offset: usize,
    pub(crate) _marker: PhantomData<T>,
}

impl<T: Rattle, const REVERSED: bool> Rattler<T, REVERSED> {
    pub fn new() -> Self {
        Self {
            interval_ms: T::INTERVAL.as_millis().max(1) as u64,
            offset: 0,
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

    fn index_at_elapsed(&self, elapsed: Duration) -> usize {
        let len = T::FRAMES.len();
        let base = frame_at(elapsed.as_millis() as u64, self.interval_ms, len);
        let shifted = if len == 0 {
            0
        } else {
            (base + self.offset) % len
        };
        apply_direction::<REVERSED>(shifted, len)
    }

    pub fn frames(&self, index: usize) -> &'static [&'static str] {
        let len = T::FRAMES.len();
        if len == 0 {
            return &[];
        }
        let shifted = (index + self.offset) % len;
        let idx = apply_direction::<REVERSED>(shifted, len);
        &T::FRAMES[idx]
    }

    pub fn frame(&self, index: usize) -> &'static str {
        self.frames(index)[0]
    }

    pub fn frames_at(&self, elapsed: Duration) -> &'static [&'static str] {
        let idx = self.index_at_elapsed(elapsed);
        &T::FRAMES[idx]
    }

    pub fn frame_at(&self, elapsed: Duration) -> &'static str {
        self.frames_at(elapsed)[0]
    }

    #[cfg(feature = "std")]
    pub fn current_frames(&self) -> &'static [&'static str] {
        self.frames_at(clock::elapsed())
    }

    #[cfg(feature = "std")]
    pub fn current_frame(&self) -> &'static str {
        self.current_frames()[0]
    }

    #[cfg(feature = "std")]
    pub fn index(&self) -> usize {
        self.index_at_elapsed(clock::elapsed())
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

    pub fn into_ticked(self) -> TickedRattler<T, REVERSED> {
        TickedRattler {
            interval_ms: self.interval_ms,
            offset: self.offset,
            current: 0,
            _marker: PhantomData,
        }
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
            interval_ms: self.interval_ms,
            offset: self.offset,
            _marker: PhantomData,
        }
    }
}

impl<T: Rattle> Rattler<T, true> {
    pub fn reverse(self) -> Rattler<T, false> {
        Rattler {
            interval_ms: self.interval_ms,
            offset: self.offset,
            _marker: PhantomData,
        }
    }
}

#[cfg(feature = "std")]
impl<T: Rattle, const REVERSED: bool> Iterator for Rattler<T, REVERSED> {
    type Item = &'static [&'static str];

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.current_frames())
    }
}
