# quad-gif

Display looping GIF animations with Macroquad.

The animation will loop forever, regardless of how many iterations are set in
the file.

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
    let mut animation = quad_gif::GifAnimation::load("animation.gif").await;

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
