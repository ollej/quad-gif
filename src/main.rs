use macroquad::prelude::*;
use std::fs::File;

struct AnimationFrame {
    image: Image,
    delay: f32,
}

impl AnimationFrame {
    fn texture(&self) -> Texture2D {
        Texture2D::from_image(&self.image)
    }
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

    let orig_x = screen_width() / 2. - decoder.width() as f32 / 2.;
    let orig_y = screen_height() / 2. - decoder.height() as f32 / 2.;

    let mut frames: Vec<AnimationFrame> = Vec::new();
    while let Some(frame) = decoder.read_next_frame().unwrap() {
        // TODO: Care about disposal method
        let frame_image = Image {
            bytes: frame.buffer.to_vec(),
            width: frame.width,
            height: frame.height,
        };
        frames.push(AnimationFrame {
            image: frame_image,
            delay: frame.delay as f32 / 100.,
        });
    }

    let mut frame_index = 0;
    loop {
        clear_background(WHITE);
        let animation_frame = frames.get(frame_index).unwrap();
        draw_texture_ex(
            animation_frame.texture(),
            orig_x,
            orig_y,
            WHITE,
            DrawTextureParams::default(),
        );
        frame_index = if frame_index == frames.len() - 1 {
            0
        } else {
            frame_index + 1
        };

        next_frame().await
    }
}
