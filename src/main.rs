use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};

use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;

const OPENGL_VERSION: OpenGL = OpenGL::V3_2;
const SCREEN_SIZE: [u32; 2] = [1000, 800];

fn main() {
    let mut window: Window = WindowSettings::new("Sketchbook", SCREEN_SIZE)
        .graphics_api(OPENGL_VERSION)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut events = Events::new(EventSettings::new());

    // Runloop
    while let Some(event) = events.next(&mut window) {
        if let Some(args) = event.render_args() {
            // Render app
        }

        if let Some(args) = event.update_args() {
            // Update app
        }
    }
}
