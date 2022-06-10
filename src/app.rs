use crate::palette;
use opengl_graphics::GlGraphics;
use piston::RenderArgs;

pub struct App {
    gl: GlGraphics,
}

impl App {
    pub fn new(gl: GlGraphics) -> Self {
        App { gl }
    }

    pub fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        // let square = rectangle::square(0.0, 0.0, 50.0);
        // let (x, y) = (args.window_size[0] / 2.0, args.window_size[1] / 2.0);

        self.gl.draw(args.viewport(), |c, gl| {
            clear(palette::DARK, gl);
        });
    }
}
