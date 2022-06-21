use macroquad::prelude::*;
use rgb::ComponentBytes;

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

#[macroquad::main(window_conf)]
async fn main() {
    let gif = load_file("animation.gif")
        .await
        .expect("Couldn't load file");
    let mut options = gif::DecodeOptions::new();
    options.set_color_output(gif::ColorOutput::Indexed);
    let mut decoder = options.read_info(&*gif).unwrap();
    let mut screen = gif_dispose::Screen::new_decoder(&decoder);

    let orig_x = screen_width() / 2. - decoder.width() as f32 / 2.;
    let orig_y = screen_height() / 2. - decoder.height() as f32 / 2.;

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

    let mut frame_index: usize = 0;
    let mut elapsed_time: f32 = 0.;
    set_default_camera();
    loop {
        #[cfg(not(target_arch = "wasm32"))]
        if is_key_pressed(KeyCode::Q) | is_key_pressed(KeyCode::Escape) {
            break;
        }

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
