pub(crate) fn apply_direction<const REVERSED: bool>(index: usize, len: usize) -> usize {
    if len == 0 {
        0
    } else {
        if REVERSED { len - 1 - index } else { index }
    }
}

pub(crate) fn frame_at(elapsed: u64, interval: u64, len: usize) -> usize {
    if len == 0 {
        0
    } else {
        ((elapsed / interval.max(1)) as usize) % len
    }
}
