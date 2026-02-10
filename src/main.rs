#![windows_subsystem = "windows"]
use clap::Parser;
use macroquad::prelude::*;
use quad_gif;

fn window_conf() -> Conf {
    Conf {
        window_title: "quad-gif".to_owned(),
        fullscreen: true,
        ..Default::default()
    }
}

/// Display a looping GIF animation
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Which filter mode to use
    #[arg(short, long, default_value = "linear")]
    filter_mode: FilterModeExt,

    /// Filename of the GIF
    #[arg(default_value = "animation.gif")]
    filename: String,
}

#[derive(Clone, Debug)]
pub struct FilterModeExt(FilterMode);

impl Into<FilterMode> for FilterModeExt {
    fn into(self) -> FilterMode {
        self.0
    }
}

impl clap::ValueEnum for FilterModeExt {
    fn value_variants<'a>() -> &'a [FilterModeExt] {
        &[
            FilterModeExt(FilterMode::Linear),
            FilterModeExt(FilterMode::Nearest),
        ]
    }

    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        match self.0 {
            FilterMode::Linear => Some(clap::builder::PossibleValue::new("linear")),
            FilterMode::Nearest => Some(clap::builder::PossibleValue::new("nearest")),
        }
    }
}

/// Binary to display a looping GIF animation.
///
/// The filename is required, except in debug, where it defaults to `animation.gif`.
///
/// quad-gif 0.5.0
/// Display a GIF file.
///
/// Usage: quad-gif <file>
#[macroquad::main(window_conf)]
async fn main() {
    let args = Args::parse();
    let mut animation =
        quad_gif::GifAnimation::load_with_filter_mode(args.filename, args.filter_mode.into()).await;

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
