use macroquad::prelude::*;
use rgb::ComponentBytes;
#[cfg(not(debug_assertions))]
use std::{env, process};

pub struct GifAnimation {
    frames: Vec<AnimationFrame>,
    width: u16,
    height: u16,
    current_frame: usize,
    elapsed_time: f32,
}

impl GifAnimation {
    fn new(frames: Vec<AnimationFrame>, width: u16, height: u16) -> Self {
        Self {
            frames,
            width,
            height,
            current_frame: 0,
            elapsed_time: 0.,
        }
    }

    async fn load(filename: String) -> Self {
        let file_bytes = load_file(&filename).await.expect("Couldn't load file");
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

    fn draw(&self) {
        self.draw_at(self.pos_x(), self.pos_y());
    }

    fn draw_at(&self, pos_x: f32, pos_y: f32) {
        draw_texture_ex(
            self.frame().texture,
            pos_x,
            pos_y,
            WHITE,
            DrawTextureParams::default(),
        );
    }

    fn tick(&mut self) {
        self.elapsed_time += get_frame_time();
        if self.elapsed_time > self.frame().delay {
            self.current_frame = if self.current_frame == self.frames.len() - 1 {
                0
            } else {
                self.current_frame + 1
            };
            self.elapsed_time = 0.0;
        }
    }

    fn frame(&self) -> &AnimationFrame {
        self.frames.get(self.current_frame).unwrap()
    }
}

#[derive(Debug)]
struct AnimationFrame {
    texture: Texture2D,
    delay: f32,
}

fn window_conf() -> Conf {
    Conf {
        window_title: "quad-gif".to_owned(),
        fullscreen: true,
        ..Default::default()
    }
}

#[cfg(debug_assertions)]
fn read_filename() -> String {
    "animation.gif".to_string()
}

#[cfg(not(debug_assertions))]
fn read_filename() -> String {
    env::args().nth(1).unwrap_or_else(|| explain_usage())
}

#[cfg(not(debug_assertions))]
fn explain_usage() -> ! {
    println!("Display a GIF file.\n\nUsage: quad-gif <file>");
    process::exit(1)
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut gif = GifAnimation::load(read_filename()).await;

    set_default_camera();
    loop {
        #[cfg(not(target_arch = "wasm32"))]
        if is_key_pressed(KeyCode::Q) | is_key_pressed(KeyCode::Escape) {
            break;
        }

        clear_background(WHITE);
        gif.draw();
        gif.tick();

        next_frame().await
    }
}
