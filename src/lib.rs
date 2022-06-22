//! Simple crate to load and draw a GIF animation using Macroquad.
//!
//! The animation will loop forever, regardless of how many iterations are set in
//! the file.
//!
//! ```rust
//! use macroquad::prelude::*;
//! use quad_gif;
//!
//! #[macroquad::main("quad-gif")]
//! async fn main() {
//!     let mut animation = quad_gif::GifAnimation::load("animation.gif".to_string()).await;
//!
//!     clear_background(WHITE);
//!     loop {
//!         animation.draw();
//!         animation.tick();
//!         next_frame().await
//!     }
//! }
//! ```

use macroquad::prelude::*;
use rgb::ComponentBytes;

/// Struct containing textures to display every frame.
pub struct GifAnimation {
    frames: Vec<AnimationFrame>,
    pub width: u16,
    pub height: u16,
    current_frame: usize,
    elapsed_time: f32,
    paused: bool,
}

impl GifAnimation {
    /// Instantiate with a vector of [`AnimationFrame`], width and height.
    ///
    /// Can be used to create an animation from your own textures instead
    /// of loading a GIF file.
    ///
    /// [`AnimationFrame`]: struct.AnimationFrame
    pub fn new(frames: Vec<AnimationFrame>, width: u16, height: u16) -> Self {
        Self {
            frames,
            width,
            height,
            current_frame: 0,
            elapsed_time: 0.,
            paused: false,
        }
    }

    /// Load and decode a GIF file using Macroquad.
    ///
    /// ```rust
    /// let mut gif_animation = GifAnimation::load("filename.gif").await;
    /// ```
    pub async fn load(filename: String) -> Self {
        let file_bytes = load_file(&filename).await.expect("Couldn't load file");
        Self::from_gif_bytes(&file_bytes)
    }

    /// Instantiate a new `GifAnimation` from bytes.
    ///
    /// ```rust
    /// let bytes: [u8] = ...
    /// let mut gif_animation = GifAnimation::from_gif_bytes(&bytes);
    /// ```
    pub fn from_gif_bytes(file_bytes: &[u8]) -> GifAnimation {
        let (frames, width, height) = Self::decode_gif(&file_bytes);
        GifAnimation::new(frames, width, height)
    }

    fn decode_gif(file: &[u8]) -> (Vec<AnimationFrame>, u16, u16) {
        let mut options = gif::DecodeOptions::new();
        options.set_color_output(gif::ColorOutput::Indexed);
        let mut decoder = options.read_info(&*file).unwrap();
        let mut screen = gif_dispose::Screen::new_decoder(&decoder);

        let mut frames: Vec<AnimationFrame> = Vec::new();
        while let Some(frame) = decoder.read_next_frame().unwrap() {
            screen.blit_frame(&frame).expect("Couldn't blit frame");
            let (pixels, frame_width, frame_height) = screen.pixels.as_contiguous_buf();
            frames.push(AnimationFrame {
                texture: Texture2D::from_rgba8(
                    frame_width as u16,
                    frame_height as u16,
                    pixels.as_bytes(),
                ),
                delay: frame.delay as f32 / 100.,
            });
        }
        (frames, decoder.width(), decoder.height())
    }

    fn pos_x(&self) -> f32 {
        screen_width() / 2. - self.width as f32 / 2.
    }

    fn pos_y(&self) -> f32 {
        screen_height() / 2. - self.height as f32 / 2.
    }

    /// Draw the texture of the current frame at the middle of the screen.
    ///
    /// ```rust
    /// gif_animation.draw();
    /// ```
    pub fn draw(&self) {
        self.draw_at(self.pos_x(), self.pos_y());
    }

    /// Draw the texture of the current frame at given X/Y position.
    ///
    /// ```rust
    /// gif_animation.draw_at(42.0, 47.0);
    /// ```
    pub fn draw_at(&self, pos_x: f32, pos_y: f32) {
        draw_texture_ex(
            self.frame().texture,
            pos_x,
            pos_y,
            WHITE,
            DrawTextureParams::default(),
        );
    }

    /// Update method that needs to be called in the loop to
    /// advance to next frame when necessary.
    ///
    /// ```rust
    /// gif_animation.tick();
    /// ```
    pub fn tick(&mut self) {
        if !self.paused {
            self.elapsed_time += get_frame_time();
            if self.elapsed_time > self.frame().delay {
                self.advance_frame();
            }
        }
    }

    /// Toggle whether the animation should be playing or be paused.
    ///
    /// ```rust
    /// gif_animation.toggle_paused();
    /// ```
    pub fn toggle_paused(&mut self) {
        self.paused ^= true;
    }

    fn frame(&self) -> &AnimationFrame {
        self.frames.get(self.current_frame).unwrap()
    }

    fn advance_frame(&mut self) {
        self.current_frame = if self.current_frame == self.frames.len() - 1 {
            0
        } else {
            self.current_frame + 1
        };
        self.elapsed_time = 0.0;
    }
}

/// Struct for a single frame. Contains the texture to draw,
/// and a delay for how many seconds the frame should show before
/// advancing to the next frame.
#[derive(Debug)]
pub struct AnimationFrame {
    texture: Texture2D,
    delay: f32,
}
