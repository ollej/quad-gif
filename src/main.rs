use macroquad::prelude::*;
use std::fs::File;

#[macroquad::main("quad-gif")]
async fn main() {
    debug!("Hello, world!");

    //let input = load_file("animation.gif")
    //    .await
    //    .expect("Couldn't load file");
    let input = File::open("animation.gif").unwrap();
    let mut options = gif::DecodeOptions::new();
    options.set_color_output(gif::ColorOutput::RGBA);
    let mut decoder = options.read_info(input).unwrap();

    let orig_x = screen_width() / 2. - decoder.width() as f32 / 2.;
    let orig_y = screen_height() / 2. - decoder.height() as f32 / 2.;

    while let Some(frame) = decoder.read_next_frame().unwrap() {
        clear_background(WHITE);
        let image_texture = Texture2D::from_rgba8(frame.width, frame.height, &frame.buffer);
        draw_texture_ex(
            image_texture,
            orig_x + frame.left as f32,
            orig_y + frame.top as f32,
            WHITE,
            DrawTextureParams::default(),
        );

        next_frame().await
    }
}
