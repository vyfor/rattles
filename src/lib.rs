#![cfg_attr(not(feature = "std"), no_std)]

#[macro_use]
mod macros;
#[cfg(feature = "std")]
mod clock;
pub mod presets;
mod size;
mod ticked;
mod timed;
mod util;

use core::time::Duration;

pub use size::Size;
pub use ticked::TickedRattler;
pub use timed::Rattler;

pub mod prelude {
    pub use crate::{Rattle, Rattler, Size, TickedRattler, presets::prelude::*};
}

pub trait Rattle: Copy + Default {
    const SIZE: Size;
    const INTERVAL: Duration;
    const FRAMES: &'static [&'static [&'static str]];
}
