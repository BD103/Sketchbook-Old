use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::window::WindowSettings;
use piston::{RenderEvent, UpdateEvent};
use sketchbook::App;

const OPENGL_VERSION: OpenGL = OpenGL::V3_2;
const SCREEN_SIZE: [u32; 2] = [1000, 600];

fn main() {
    let mut window: Window = WindowSettings::new("Sketchbook", SCREEN_SIZE)
        .graphics_api(OPENGL_VERSION)
        .resizable(false)
        .exit_on_esc(true)
        .controllers(false)
        .build()
        .unwrap();

    let mut app = App::new(GlGraphics::new(OPENGL_VERSION));

    let mut events = Events::new(EventSettings::new());
    while let Some(event) = events.next(&mut window) {
        if let Some(args) = event.render_args() {
            app.render(&args);
        }

        if let Some(_args) = event.update_args() {
            // Update
        }
    }
}
