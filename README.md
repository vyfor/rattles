# 🪇 Rattles

![Demo](./.github/assets/demo.gif)

**Rattles** is a terminal spinner library for Rust. It is equipped with extensive preset library and lets you define custom spinners at compile time. A work in progress.

- **Rattles** is *compile-time first*; all spinner data is baked in at compile time aiming for minimal overhead.

- **Rattles** is *minimal*; it makes no assumptions about how the output will be used, and just works with no prior configuration required. It also has zero dependencies.

## Philosophy

Most spinner libraries are built as actors or widgets. Rattles is not. It has no runtime, no lifecycle, and doesn't need to be integrated.

Spinners can be constructed directly in the render loop with negligible cost. The result is a library that gets out of your way.

## Quick Start

```sh
cargo add rattles
```

### Minimal example

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

The interval of the animation can be configured with `set_interval(...)` method, and the direction can be flipped with `reverse()`.

### Custom keyframes

```rust
rattle!(
    Custom, // struct name
    custom, // method name
    1,      // number of rows (string width)
    100,    // interval in milliseconds
    ["⣾", "⣷", "⣯", "⣟", "⣻", "⣽", "⣾"] // keyframes
)
```

### `no_std` support

`rattles` enables the `std` feature by default. To use it in `no_std` mode:

```sh
cargo add rattles --no-default-features
```

In `no_std` mode, use time-driven or index-based APIs.
The following APIs are only available with `std` enabled:

- `Rattler::current_frames()`
- `Rattler::current_frame()`
- `Rattler::index()`

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

There's also a minimal no_std-oriented usage example found [here](./examples/no_std.rs).

## Acknowledgements

- [sindresorhus/cli-spinners](https://github.com/sindresorhus/cli-spinners)
- [gunnargray-dev/unicode-animations](https://github.com/gunnargray-dev/unicode-animations)
