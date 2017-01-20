extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;
extern crate one_d;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use one_d::maze::{Maze, MazeSpace};

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    maze: Maze,
}

impl App {
    fn new(opengl: glutin_window::OpenGL, width: usize, height: usize) -> App {
        App {
            gl: GlGraphics::new(opengl),
            maze: Maze::new(width, height),
        }
    }

    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
        const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 0.8];

        let width = args.width as f64;
        let square_size = width / (self.maze.width() as f64);

        let square = rectangle::square(0.0, 0.0, square_size);
        let maze = &self.maze;

        self.gl.draw(args.viewport(), move |c, gl| {
            // Clear the screen.
            clear(BLACK, gl);

            for (y, row) in maze.rows().enumerate() {
                for (x, wall) in row.enumerate() {
                    match wall {
                        MazeSpace::Open => {
                            let (x, y) = (x as f64, y as f64);
                            let transform = c.transform
                                .trans(x * square_size, y * square_size);

                            // Draw rectangle
                            rectangle(WHITE, square, transform, gl);
                        },
                        MazeSpace::Goal => {
                            let (x, y) = (x as f64, y as f64);
                            let transform = c.transform
                                .trans(x * square_size, y * square_size);

                            // Draw rectangle
                            rectangle(WHITE, square, transform, gl);
                            rectangle(GREEN, square, transform, gl);
                        },
                        MazeSpace::Wall => {},
                    }
                }
            }
        });
    }

    fn update(&mut self, args: &UpdateArgs) {}
}

fn main() {
    let opengl = OpenGL::V3_2;
    let window_width = 640;
    let window_height = 640;

    // Create an Glutin window.
    let mut window: Window = WindowSettings::new("oned", [window_width, window_height])
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut app = App::new(opengl, 5, 5);

    let mut events = window.events();
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            app.render(&r);
        }

        if let Some(u) = e.update_args() {
            app.update(&u);
        }
    }
}
