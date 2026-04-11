#![cfg_attr(not(feature = "std"), no_std)]

#[macro_use]
mod macros;
#[cfg(feature = "std")]
mod clock;
pub mod presets;
mod size;
mod state;

pub use size::Size;
pub use state::{Rattle, Rattler};

pub mod prelude {
    pub use crate::{Rattle, Rattler, Size, presets::prelude::*};
}
