#![no_std]

use core::time::Duration;
use rattles::presets::prelude as presets;

// a simple clock for demonstration purposes
struct UserClock {
    elapsed_ms: u64,
}

impl UserClock {
    fn new() -> Self {
        Self { elapsed_ms: 0 }
    }

    fn elapsed(&self) -> Duration {
        Duration::from_millis(self.elapsed_ms)
    }

    fn tick_ms(&mut self, delta_ms: u64) {
        self.elapsed_ms += delta_ms;
    }
}

fn using_user_clock() {
    let rattle = presets::dots();
    let mut clock = UserClock::new();

    for _ in 0..8 {
        let elapsed = clock.elapsed();
        let __frame = rattle.frame_at(elapsed);

        // do something with frame

        clock.tick_ms(80);
    }
}

fn using_frame_index() {
    let rattle = presets::dots();

    for i in 0..8 {
        let _frame = rattle.frame(i);

        // do something with frame
    }
}

fn main() {
    using_user_clock();
    using_frame_index();
}
