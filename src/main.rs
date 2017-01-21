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

struct Point(usize, usize);

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    maze: Maze,
    position: Point,
}

impl App {
    fn new(opengl: glutin_window::OpenGL, width: usize, height: usize) -> App {
        App {
            gl: GlGraphics::new(opengl),
            maze: Maze::new(width, height),
            position: Point(0, 0),
        }
    }

    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
        const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 0.8];
        const BLUE: [f32; 4] = [0.0, 0.0, 1.0, 0.8];

        let width = args.width as f64;
        let square_size = width / (self.maze.width() as f64);

        let square = rectangle::square(0.0, 0.0, square_size);
        let maze = &self.maze;
        let position = &self.position;

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

            let player_size = square_size * 0.75;
            let player = rectangle::square(0.0, 0.0, player_size);
            let &Point(x, y) = position;
            let (x, y) = (x as f64, y as f64);
            let transform = c.transform
                .trans((x + 0.125) * square_size, (y + 0.125) * square_size);

            rectangle(BLUE, player, transform, gl);
        });
    }

    fn attempt_move(&mut self, x_step: i32, y_step: i32) {
        let Point(x, y) = self.position;
        if self.maze.is_goal(x, y) { return; }

        let x = if x > 0 || x_step >= 0 { (x as i32 + x_step) as usize } else { x };
        let y = if y > 0 || y_step >= 0 { (y as i32 + y_step) as usize} else { y };

        if self.maze.is_valid_space(x, y) {
            self.position = Point(x, y);
        }
    }

    fn handle(&mut self, button: Button) {
        use piston::input::keyboard::Key;

        match button {
            Button::Keyboard(Key::F) => self.attempt_move(1, 0),
            Button::Keyboard(Key::S) => self.attempt_move(-1, 0),
            Button::Keyboard(Key::D) => self.attempt_move(0, 1),
            Button::Keyboard(Key::E) => self.attempt_move(0, -1),
            _ => {},
        }
    }
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

    let mut app = App::new(opengl, 13, 13);

    let mut events = window.events();
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            app.render(&r);
        }

        if let Some(button) = e.release_args() {
            app.handle(button);
        }
    }
}
