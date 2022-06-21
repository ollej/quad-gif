use macroquad::prelude::*;
use std::fs::File;

#[derive(Debug)]
struct AnimationFrame {
    texture: Texture2D,
    delay: f32,
}

#[macroquad::main("quad-gif")]
async fn main() {
    //let input = load_file("animation.gif")
    //    .await
    //    .expect("Couldn't load file");
    let input = File::open("animation.gif").unwrap();
    let mut options = gif::DecodeOptions::new();
    options.set_color_output(gif::ColorOutput::RGBA);
    let mut decoder = options.read_info(input).unwrap();

    let image_width = decoder.width() as u32;
    let image_height = decoder.height() as u32;
    let orig_x = screen_width() / 2. - image_width as f32 / 2.;
    let orig_y = screen_height() / 2. - image_height as f32 / 2.;

    let mut frames: Vec<AnimationFrame> = Vec::new();
    while let Some(frame) = decoder.read_next_frame().unwrap() {
        frames.push(AnimationFrame {
            texture: Texture2D::from_rgba8(frame.width, frame.height, &frame.buffer),
            delay: frame.delay as f32 / 100.,
        });
    }

    let mut frame_index: usize = 0;
    let mut elapsed_time: f32 = 0.;
    set_default_camera();
    loop {
        clear_background(WHITE);
        let animation_frame = frames.get(frame_index).unwrap();
        draw_texture_ex(
            animation_frame.texture,
            orig_x,
            orig_y,
            WHITE,
            DrawTextureParams::default(),
        );

        elapsed_time += get_frame_time();
        if elapsed_time > animation_frame.delay {
            frame_index = if frame_index == frames.len() - 1 {
                0
            } else {
                frame_index + 1
            };
            elapsed_time = 0.0;
        }
        next_frame().await
    }
}
