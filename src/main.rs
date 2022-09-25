#![windows_subsystem = "windows"]
use macroquad::prelude::*;
use quad_gif;
use std::env;
#[cfg(not(debug_assertions))]
use std::process;

fn window_conf() -> Conf {
    Conf {
        window_title: "quad-gif".to_owned(),
        fullscreen: true,
        ..Default::default()
    }
}

fn get_filename() -> String {
    env::args().nth(1).unwrap_or_else(|| default_filename())
}

#[cfg(debug_assertions)]
fn default_filename() -> String {
    "animation.gif".to_string()
}

#[cfg(not(debug_assertions))]
fn default_filename() -> String {
    explain_usage()
}

#[cfg(not(debug_assertions))]
fn explain_usage() -> ! {
    println!("Display a GIF file.\n\nUsage: quad-gif <file>");
    process::exit(1)
}

/// Binary to display a looping GIF animation.
///
/// The filename is required, except in debug, where it defaults to `animation.gif`.
///
/// quad-gif 0.2.0
/// Display a GIF file.
///
/// Usage: quad-gif <file>
#[macroquad::main(window_conf)]
async fn main() {
    let mut animation = quad_gif::GifAnimation::load(get_filename()).await;

    loop {
        #[cfg(not(target_arch = "wasm32"))]
        if is_key_pressed(KeyCode::Q) | is_key_pressed(KeyCode::Escape) {
            break;
        }
        if is_key_pressed(KeyCode::P) | is_key_pressed(KeyCode::Space) {
            animation.toggle_paused();
        }

        clear_background(WHITE);
        animation.draw();
        animation.tick();

        next_frame().await
    }
}
