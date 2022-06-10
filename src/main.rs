use macroquad::prelude::*;
use sketchbook::palette;

fn config() -> Conf {
    Conf {
        window_title: "Sketchbook".to_string(),
        window_width: 1000,
        window_height: 600,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(config)]
async fn main() {
    loop {
        clear_background(palette::DARK);
        next_frame().await;
    }
}
