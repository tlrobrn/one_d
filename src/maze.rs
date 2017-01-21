extern crate rand;

use self::rand::{thread_rng, Rng};
use dsets::DisjointSets;
use std::fmt::{self, Formatter, Display};

#[derive(Copy, Clone, PartialEq)]
pub enum MazeSpace {
    Open,
    Wall,
    Goal,
}

pub struct Maze {
    width: usize,
    height: usize,
    spaces: Vec<MazeSpace>,
    goal_x: usize,
    goal_y: usize,
    goal_distance: usize,
}

pub struct MazeIterator<'a> {
    maze: &'a Maze,
    current_index: usize,
    ending_index: usize,
    step: usize,
}

impl<'a> MazeIterator<'a> {
    fn row_iterator(maze: &'a Maze, row: usize) -> MazeIterator {
        let step = 1;
        let start = maze.width * row;
        let end = maze.width * (row + 1) - step;

        MazeIterator {
            maze: maze,
            current_index: start,
            ending_index: end,
            step: step,
        }
    }

    fn column_iterator(maze: &'a Maze, column: usize) -> MazeIterator {
        let step = maze.width;

        MazeIterator {
            maze: maze,
            current_index: column,
            ending_index: (maze.height - 1) * step + column,
            step: step,
        }
    }
}

impl<'a> Iterator for MazeIterator<'a> {
    type Item = MazeSpace;

    fn next(&mut self) -> Option<MazeSpace> {
        if self.current_index > self.ending_index {
            return None;
        }
        let result = Some(self.maze.spaces[self.current_index]);
        self.current_index += self.step;

        result
    }
}

pub struct MazeRowIterator<'a> {
    maze: &'a Maze,
    current_row: usize,
}

impl<'a> MazeRowIterator<'a> {
    fn new(maze: &'a Maze) -> MazeRowIterator {
        MazeRowIterator {
            maze: maze,
            current_row: 0,
        }
    }
}

impl<'a> Iterator for MazeRowIterator<'a> {
    type Item = MazeIterator<'a>;

    fn next(&mut self) -> Option<MazeIterator<'a>> {
        if self.current_row >= self.maze.height {
            return None;
        }

        let result = Some(self.maze.row(self.current_row));
        self.current_row += 1;

        result
    }
}


impl Maze {
    pub fn new(width: usize, height: usize) -> Maze {
        let width_with_spaces = 2 * width - 1;
        let height_with_spaces = 2 * height - 1;
        let size = width_with_spaces * height_with_spaces;

        let mut maze = Maze {
            width: width_with_spaces,
            height: height_with_spaces,
            spaces: vec![MazeSpace::Open; size],
            goal_x: 0,
            goal_y: 0,
            goal_distance: 0,
        };

        let mut dsets = DisjointSets::new(size);
        let (east, south) = maze.initialize_spaces();

        for i in 0..east.len() {
            maze.try_wall(&mut dsets, east[i], true);
            maze.try_wall(&mut dsets, south[i], false);
        }

        maze.solve();

        maze
    }

    pub fn is_valid_space(&self, x: usize, y: usize) -> bool {
        let index = self.index_for(x, y);
        index < self.spaces.len() && self.spaces[index] != MazeSpace::Wall && x < self.width &&
        y < self.height
    }

    pub fn is_goal(&self, x: usize, y: usize) -> bool {
        let index = self.index_for(x, y);
        self.spaces[index] == MazeSpace::Goal
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn size(&self) -> (usize, usize) {
        ((self.width + 1) / 2, (self.height + 1) / 2)
    }

    pub fn row(&self, row: usize) -> MazeIterator {
        MazeIterator::row_iterator(&self, row)
    }

    pub fn column(&self, column: usize) -> MazeIterator {
        MazeIterator::column_iterator(&self, column)
    }

    pub fn rows(&self) -> MazeRowIterator {
        MazeRowIterator::new(&self)
    }

    fn initialize_spaces(&mut self) -> (Vec<usize>, Vec<usize>) {
        let mut east = vec![];
        let mut south = vec![];

        for y in 0..self.height {
            for x in 0..self.width {
                let i = x + y * self.width;
                let is_wall = (x % 2) == 1 || (y % 2) == 1;

                self.spaces[i] = if is_wall {
                    MazeSpace::Wall
                } else {
                    MazeSpace::Open
                };
                if !is_wall {
                    east.push(i);
                    south.push(i);
                }
            }
        }

        let mut rng = thread_rng();
        rng.shuffle(&mut east);
        rng.shuffle(&mut south);

        (east, south)
    }

    fn try_wall(&mut self, mut dsets: &mut DisjointSets, index: usize, is_east: bool) {
        let step = match self.coordinates(index) {
            (x, _) if is_east && (self.width > 1 && x < self.width - 2) => 1,
            (_, y) if (self.height > 1 && y < self.height - 2) => self.width,
            _ => return,
        };

        if dsets.find_root(index) != dsets.find_root(index + step * 2) {
            self.spaces[index + step] = MazeSpace::Open;
            dsets.set_union(index, index + step);
            dsets.set_union(index, index + step * 2);
        }
    }

    fn coordinates(&self, index: usize) -> (usize, usize) {
        (index % self.width, index / self.width)
    }

    fn index_for(&self, x: usize, y: usize) -> usize {
        x + y * self.width
    }

    fn solve(&mut self) {
        let mut visited = vec![false; self.spaces.len()];
        self.dfs(&mut visited, 0, 0, 0);
        let index = self.index_for(self.goal_x, self.goal_y);
        self.spaces[index] = MazeSpace::Goal;
    }

    fn dfs(&mut self, mut visited: &mut [bool], x: usize, y: usize, current_length: usize) {
        let index = self.index_for(x, y);
        visited[index] = true;

        // left neighbor
        if x > 0 {
            let neighbor = self.index_for(x - 1, y);
            if !visited[neighbor] && self.spaces[neighbor] != MazeSpace::Wall {
                self.dfs(&mut visited, x - 1, y, current_length + 1);
            }
        }

        // right neighbor
        if x < self.width - 1 {
            let neighbor = self.index_for(x + 1, y);
            if !visited[neighbor] && self.spaces[neighbor] != MazeSpace::Wall {
                self.dfs(&mut visited, x + 1, y, current_length + 1);
            }
        }

        // up neighbor
        if y > 0 {
            let neighbor = self.index_for(x, y - 1);
            if !visited[neighbor] && self.spaces[neighbor] != MazeSpace::Wall {
                self.dfs(&mut visited, x, y - 1, current_length + 1);
            }
        }

        // down neighbor
        if y < self.height - 1 {
            let neighbor = self.index_for(x, y + 1);
            if !visited[neighbor] && self.spaces[neighbor] != MazeSpace::Wall {
                self.dfs(&mut visited, x, y + 1, current_length + 1);
            }
        }

        // set max_length if appropriate
        if (current_length > self.goal_distance) ||
           (current_length == self.goal_distance && y > self.goal_y) ||
           (current_length == self.goal_distance && y == self.goal_y && x < self.goal_x) {
            self.goal_distance = current_length;
            self.goal_x = x;
            self.goal_y = y;
        }

        visited[index] = false;
    }
}

impl Display for Maze {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let mut s = String::new();
        for _ in 0..(self.width + 1) {
            s.push('X');
        }
        for y in 0..self.height {
            s.push('X');
            s.push('\n');
            s.push('X');
            for x in 0..self.width {
                s.push(match self.spaces[x + y * self.width] {
                    MazeSpace::Wall => 'X',
                    MazeSpace::Open => ' ',
                    MazeSpace::Goal => '*',
                });
            }
        }
        s.push('X');
        s.push('\n');
        s.push('X');
        for _ in 0..(self.width + 1) {
            s.push('X');
        }

        let (width, height) = self.size();
        write!(f, "{}x{} Maze:\n{}", width, height, s)
    }
}
