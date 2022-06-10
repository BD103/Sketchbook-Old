use macroquad::prelude::*;
use sketchbook::*;

fn config() -> Conf {
    Conf {
        window_title: "Sketchbook".to_string(),
        window_width: SCREEN_WIDTH,
        window_height: SCREEN_HEIGHT,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(config)]
async fn main() {
    let mut app = App::new();

    'runloop: loop {
        app.update();

        clear_background(palette::DARK);

        draw_text("Sketchbook by BD103", 10.0, 20.0, 30.0, palette::LIGHT);
        app.draw_level();

        if let Some(c) = get_last_key_pressed() {
            match c {
                KeyCode::Escape => break 'runloop,
                _ => {}
            }
        }

        next_frame().await;
    }
}
