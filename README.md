# 🪇 Rattles

![Demo](./.github/assets/demo.gif)

**Rattles** is a minimal, dependency-free terminal spinner library for Rust. It makes no assumptions about how the output will be used.

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
    Note that `TickedRattler` is stateful and must be stored outside the render loop.

## Presets

Built-in presets are organized by category:

- `presets::arrows`
- `presets::ascii`
- `presets::braille`
- `presets::emoji`

A prelude is available `rattles::prelude`.

## Examples

- a minimal ratatui [example](./examples/ratatui.rs)
- a no_std-oriented [example](./examples/no_std.rs)

## Acknowledgements

Rattles includes spinners sourced from these wonderful projects. We gratefully acknowledge their work.

- [sindresorhus/cli-spinners](https://github.com/sindresorhus/cli-spinners)
- [gunnargray-dev/unicode-animations](https://github.com/gunnargray-dev/unicode-animations)
