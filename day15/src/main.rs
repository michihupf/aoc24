use core::panic;

use aoclib::{input, output};

type Vec2D = aoclib::Vec2D<isize>;

#[derive(Debug, Clone, Copy)]
enum Move {
    Up,
    Left,
    Down,
    Right,
}

impl Move {
    fn v(&self) -> Vec2D {
        match self {
            Self::Up => Vec2D::new(0, -1),
            Self::Left => Vec2D::new(-1, 0),
            Self::Down => Vec2D::new(0, 1),
            Self::Right => Vec2D::new(1, 0),
        }
    }

    fn from(c: char) -> Move {
        match c {
            '^' => Self::Up,
            '<' => Self::Left,
            'v' => Self::Down,
            '>' => Self::Right,
            _ => unimplemented!("NOT POSSIBLE"),
        }
    }
}

#[derive(Clone)]
struct Grid {
    bot: Vec2D,
    map: Vec<Vec<char>>,
    wide: bool,
    rules: Vec<Move>,
}

const OFFSET_RIGHT: Vec2D = Vec2D::new(1, 0);
const OFFSET_LEFT: Vec2D = Vec2D::new(-1, 0);

impl Grid {
    fn new(input: &str) -> Self {
        let mut grid = Grid {
            bot: Vec2D::new(0, 0),
            map: Vec::new(),
            wide: false,
            rules: Vec::new(),
        };
        let mut parse_board = true;
        for (y, line) in input.lines().enumerate() {
            if line.is_empty() {
                parse_board = false;
            }
            if parse_board {
                let mut row = Vec::new();
                for (x, c) in line.chars().enumerate() {
                    row.push(c);
                    if c == '@' {
                        grid.bot = Vec2D::new(x as isize, y as isize)
                    }
                }
                grid.map.push(row);
            } else {
                for c in line.chars() {
                    grid.rules.push(Move::from(c));
                }
            }
        }
        grid
    }

    fn new_wide(input: &str) -> Self {
        let mut grid = Grid {
            bot: Vec2D::new(0, 0),
            map: Vec::new(),
            wide: true,
            rules: Vec::new(),
        };
        let mut parse_board = true;
        for (y, line) in input.lines().enumerate() {
            if line.is_empty() {
                parse_board = false;
            }
            if parse_board {
                let mut row = Vec::new();
                for (x, c) in line.chars().enumerate() {
                    match c {
                        '@' => {
                            row.push('@');
                            row.push('.');
                            grid.bot = Vec2D::new(2 * x as isize, y as isize);
                        }
                        'O' => {
                            row.push('[');
                            row.push(']');
                        }
                        c => {
                            row.push(c);
                            row.push(c);
                        }
                    }
                }
                grid.map.push(row);
            } else {
                for c in line.chars() {
                    grid.rules.push(Move::from(c));
                }
            }
        }
        grid
    }

    fn gps_sum(&self) -> usize {
        self.map
            .iter()
            .enumerate()
            .flat_map(|(y, v)| {
                v.iter().enumerate().map(move |(x, c)| {
                    if *c == 'O' || *c == '[' {
                        100 * y + x
                    } else {
                        0
                    }
                })
            })
            .sum()
    }

    fn move_seq(&mut self) {
        let moves = self.rules.clone();
        for m in moves {
            if self.can_move(self.bot, &m) {
                self.mv(self.bot, &m);
            }
        }
    }

    fn at(&self, pos: &Vec2D) -> char {
        assert!(pos.y >= 0);
        assert!(pos.x >= 0);
        self.map[pos.y as usize][pos.x as usize]
    }

    fn at_mut(&mut self, pos: &Vec2D) -> &mut char {
        assert!(pos.y >= 0);
        assert!(pos.x >= 0);
        &mut self.map[pos.y as usize][pos.x as usize]
    }

    /// Recursively checks if bot and boxes can be moved.
    fn can_move(&self, pos: Vec2D, m: &Move) -> bool {
        let next = pos + m.v();

        if self.wide && matches!(m, Move::Up | Move::Down) {
            match self.at(&next) {
                '#' => false,
                '[' => self.can_move(next, m) && self.can_move(next + OFFSET_RIGHT, m),
                ']' => self.can_move(next + OFFSET_LEFT, m) && self.can_move(next, m),
                _ => true,
            }
        } else {
            match self.at(&next) {
                '#' => false,
                'O' | '[' | ']' => self.can_move(next, m),
                _ => true,
            }
        }
    }

    /// Recursively moves bot and boxes from pos in m direction. If forwarded no movement occours for this object.
    fn mv(&mut self, pos: Vec2D, m: &Move) {
        let next = pos + m.v();

        if self.wide && matches!(m, Move::Up | Move::Down) {
            match self.at(&next) {
                '#' => unreachable!("Should not have encountered a wall"),
                '[' => {
                    self.mv(next, m);
                    self.mv(next + OFFSET_RIGHT, m);
                }
                ']' => {
                    self.mv(next + OFFSET_LEFT, m);
                    self.mv(next, m);
                }
                _ => {}
            };
            if pos == self.bot {
                self.bot = next;
                self.swap(&next, &pos);
            } else {
                self.swap(&next, &pos);
            }
        } else {
            match self.at(&next) {
                'O' | ']' | '[' => self.mv(next, m),
                _ => {}
            }
            if pos == self.bot {
                self.bot = next;
                self.swap(&next, &pos);
            } else {
                self.swap(&next, &pos);
            }
        }
    }

    fn swap(&mut self, p1: &Vec2D, p2: &Vec2D) {
        let tmp = self.at(p1);
        *self.at_mut(p1) = self.at(p2);
        *self.at_mut(p2) = tmp;
    }
}

fn main() {
    let input = input("input");

    // parse 1
    let grid = Grid::new(&input);
    let wide_grid = Grid::new_wide(&input);

    p1(grid);
    p2(wide_grid);
}

#[inline]
fn p1(mut grid: Grid) {
    grid.move_seq();
    output(grid.gps_sum());
}

#[inline]
fn p2(mut grid: Grid) {
    grid.move_seq();
    output(grid.gps_sum());
}
