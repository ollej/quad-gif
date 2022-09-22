# quad-gif

[![Cross-compile](https://github.com/ollej/quad-gif/actions/workflows/crosscompile.yml/badge.svg)](https://github.com/ollej/quad-gif/actions/workflows/crosscompile.yml) [![Crates.io](https://img.shields.io/crates/v/quad-gif)](https://crates.io/crates/quad-gif) [![docs.rs](https://img.shields.io/docsrs/quad-gif)](https://docs.rs/quad-gif/latest/quad_gif/) [![Crates.io](https://img.shields.io/crates/l/quad-gif)](https://opensource.org/licenses/MIT)

Display looping GIF animations with Macroquad.

The animation will loop forever, regardless of how many iterations are set in
the file.

[Documentation](https://docs.rs/quad-gif/latest/quad_gif/) on docs.rs

## Usage

There is a binary file included that can be used to show a GIF file.

```
quad-gif 0.2.0
Display a GIF file.

Usage: quad-gif <file>
```

## API usage

The library can be used in a Macroquad application to show an animation.

```rust
use macroquad::prelude::*;
use quad_gif;

#[macroquad::main("quad-gif")]
async fn main() {
    let mut animation = quad_gif::GifAnimation::load("animation.gif".to_string()).await;

    clear_background(WHITE);
    loop {
        animation.draw();
        animation.tick();
        next_frame().await
    }
}
```

## License

Copyright 2022 Olle Wreede, released under the MIT License.

## Attribution

Animated Ferris in Action by A. L. Palmer

Happy as a Rustacean at Rust Fest Berlin 2016 (www.rustfest.eu)
