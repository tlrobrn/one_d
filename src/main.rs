extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;
extern crate one_d;
extern crate rustc_serialize;
extern crate docopt;

use std::{cmp, env};
use docopt::Docopt;
use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use one_d::maze::{Maze, MazeSpace};

struct Point(usize, usize);

enum MazeView {
    Row,
    Column,
}

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    maze: Maze,
    position: Point,
    view: MazeView,
}

impl App {
    fn new(opengl: glutin_window::OpenGL, width: usize, height: usize) -> App {
        App {
            gl: GlGraphics::new(opengl),
            maze: Maze::new(width, height),
            position: Point(0, 0),
            view: MazeView::Row,
        }
    }

    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
        const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 0.8];
        const BLUE: [f32; 4] = [0.0, 0.0, 1.0, 0.8];

        let width = args.width as f64;
        let square_size = width / (cmp::max(self.maze.width(), self.maze.height()) as f64);

        let square = rectangle::square(0.0, 0.0, square_size);
        let maze = &self.maze;
        let position = &self.position;
        let view = &self.view;

        self.gl.draw(args.viewport(), move |c, gl| {
            // Clear the screen.
            clear(BLACK, gl);

            let player_size = square_size * 0.75;
            let player = rectangle::square(0.0, 0.0, player_size);
            let &Point(x, y) = position;
            let (spaces, p) = match view {
                &MazeView::Row => (maze.row(y), x as f64),
                &MazeView::Column => (maze.column(x), y as f64),
            };
            for (x, space) in spaces.enumerate() {
                match space {
                    MazeSpace::Open => {
                        let x = x as f64;
                        let transform = c.transform.trans(x * square_size, 0.0);

                        // Draw rectangle
                        rectangle(WHITE, square, transform, gl);
                    }
                    MazeSpace::Goal => {
                        let x = x as f64;
                        let transform = c.transform.trans(x * square_size, 0.0);

                        // Draw rectangle
                        rectangle(WHITE, square, transform, gl);
                        rectangle(GREEN, square, transform, gl);
                    }
                    MazeSpace::Wall => {}
                }
            }

            let transform = c.transform.trans((p + 0.125) * square_size, 0.125 * square_size);
            rectangle(BLUE, player, transform, gl);
        });
    }

    fn flip_view(&mut self) {
        self.view = match self.view {
            MazeView::Row => MazeView::Column,
            MazeView::Column => MazeView::Row,
        };
    }

    fn move_player(&mut self, step: i32) {
        match self.view {
            MazeView::Row => self.attempt_move(step, 0),
            MazeView::Column => self.attempt_move(0, step),
        }
    }

    fn attempt_move(&mut self, x_step: i32, y_step: i32) {
        let Point(x, y) = self.position;
        if self.maze.is_goal(x, y) {
            return;
        }

        let x = if x > 0 || x_step >= 0 {
            (x as i32 + x_step) as usize
        } else {
            x
        };
        let y = if y > 0 || y_step >= 0 {
            (y as i32 + y_step) as usize
        } else {
            y
        };

        if self.maze.is_valid_space(x, y) {
            self.position = Point(x, y);
        }
    }

    fn handle(&mut self, button: Button) {
        use piston::input::keyboard::Key;

        match button {
            Button::Keyboard(Key::F) => self.flip_view(),
            Button::Keyboard(Key::J) => self.move_player(-1),
            Button::Keyboard(Key::K) => self.move_player(1),
            _ => {}
        }
    }
}

const USAGE: &'static str = "
OneD Maze

Usage:
  oned
  oned <size>
  oned <width> <height>

Options:
  -h --help     Show this screen.
";

#[derive(RustcDecodable, Debug)]
struct Args {
    arg_size: usize,
    arg_width: usize,
    arg_height: usize,
}

fn main() {
    let argv: Vec<String> = env::args().collect();
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.argv(argv.into_iter()).decode())
        .unwrap_or_else(|e| e.exit());

    let (maze_width, maze_height) = match args {
        Args { arg_size: size, arg_width: w, arg_height: h } if size == 0 && w == 0 && h == 0 => {
            (5, 5)
        }
        Args { arg_size: size, arg_width: _, arg_height: _ } if size != 0 => (size, size),
        Args { arg_size: _, arg_width: w, arg_height: h } => (w, h),
    };

    let opengl = OpenGL::V3_2;
    let window_width = 640;
    let max_maze_width = cmp::max(maze_width, maze_height) * 2 - 1;
    let window_height = window_width / max_maze_width;

    let mut window: Window = WindowSettings::new("oned", [window_width as u32, window_height as u32])
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut app = App::new(opengl, maze_width, maze_height);

    println!("{}", app.maze);

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
