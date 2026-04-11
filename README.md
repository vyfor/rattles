# 🪇 Rattles

![Demo](./.github/assets/demo.gif)

**Rattles** is a terminal spinner library for Rust with an extensive preset collection and lets you define custom spinners at compile time.

- **Rattles** is *compile-time first*; all spinner data is baked in at compile time aiming for minimal overhead.

- **Rattles** is *minimal*; it makes no assumptions about how the output will be used, and just works with no prior configuration required. It also has zero dependencies.

## Philosophy

Most spinner libraries are built as actors or widgets. Rattles is neither. It has no runtime, no lifecycle, and requires no integration by default.

Spinners can be constructed directly in the render loop with negligible cost. The result is a library that gets out of your way.

## Quick Start

```sh
cargo add rattles
```

### Basic usage

```rust
use std::{io::Write, time::Duration};
use rattles::presets::prelude as presets;

fn main() {
    let rattle = presets::waverows();

    // change default interval
    // let rattle = presets::waverows().set_interval(Duration::from_millis(100));

    // reverse direction
    // let rattle = presets::waverows().reverse();

    loop {
        print!("\r{}", rattle.current_frame());
        std::io::stdout().flush().unwrap();
        std::thread::sleep(Duration::from_millis(20));
    }
}
```

### Custom keyframes

```rust
rattle!(
    Custom, // struct name
    custom, // method name
    1,      // row count (width of the spinner)
    100,    // interval in milliseconds
    ["⣾", "⣷", "⣯", "⣟", "⣻", "⣽", "⣾"] // keyframes
)
```

### `no_std`

`rattles` enables the `std` feature by default. To opt out:

```sh
cargo add rattles --no-default-features
```

Without `std`, the global clock is unavailable. Animations can still be driven three ways:

- Time-based, with an external clock: 
    ```rs
    rattle.frame_at(elapsed)
    ```
- Index-based:
    ```rs
    rattle.frame(n)
    ```
- Tick-based:
    ```rust
    let mut rattle = presets::dots().into_ticked();
    rattle.tick();
    let frame = rattle.current_frame();
    ```
    Note that `TickedRattler` is stateful and must be stored.

## Presets

Built-in presets are organized by category:

- `presets::arrows`
- `presets::ascii`
- `presets::braille`
- `presets::emoji`

A prelude is available `rattles::presets::prelude`.

## Examples

Example showcasing all presets, built with [ratatui](https://ratatui.rs/):

```bash
cargo run --example showcase
```

There's also a minimal no_std-oriented usage example found [here](./examples/no_std.rs), including:

- `TickedRattler` usage via `into_ticked()`
- elapsed-time driven usage via `frame_at(...)`
- index-driven usage via `frame(...)`

## Acknowledgements

- [sindresorhus/cli-spinners](https://github.com/sindresorhus/cli-spinners)
- [gunnargray-dev/unicode-animations](https://github.com/gunnargray-dev/unicode-animations)
