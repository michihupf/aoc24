use core::panic;
use std::collections::HashSet;

use aoclib::{input, output};

type Position = (isize, isize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl From<char> for Direction {
    fn from(value: char) -> Self {
        match value {
            '^' => Self::North,
            '<' => Self::West,
            'v' => Self::South,
            '>' => Self::East,
            _ => panic!("Not a direction."),
        }
    }
}

impl Direction {
    fn turn(&mut self) {
        unsafe {
            let i = *(self as *const Direction as *const u8);
            *self = std::mem::transmute::<u8, Direction>((i + 1) % 4);
        }
    }

    fn step(&self, current: Position) -> Position {
        let mut c = current;
        match &self {
            Direction::North => c.1 -= 1,
            Direction::East => c.0 += 1,
            Direction::South => c.1 += 1,
            Direction::West => c.0 -= 1,
        }
        c
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Guard {
    pos: Position,
    direction: Direction,
}

#[derive(Debug, Clone)]
struct Grid {
    height: usize,
    width: usize,
    inner: Vec<Vec<char>>,
}

impl Grid {
    fn at(&self, pos: Position) -> char {
        self.inner[pos.1 as usize][pos.0 as usize]
    }

    fn set(&mut self, pos: Position, c: char) {
        self.inner[pos.1 as usize][pos.0 as usize] = c;
    }

    fn inbound(&self, pos: Position) -> bool {
        (pos.1 as usize) < self.height && (pos.0 as usize) < self.width
    }

    fn obstacle(&self, pos: Position) -> bool {
        self.inbound(pos) && self.at(pos) == '#'
    }
}

impl Guard {
    /// Steps the guard.
    fn step(&mut self, grid: &Grid) -> bool {
        while self.facing_obstacle(grid) {
            self.direction.turn();
        }
        let new = self.direction.step(self.pos);
        if grid.inbound(new) {
            self.pos = new;
            true
        } else {
            // guard left the scence
            false
        }
    }

    fn facing_obstacle(&self, grid: &Grid) -> bool {
        let front = self.direction.step(self.pos);
        grid.obstacle(front)
    }
}

fn main() {
    let input = input("input");

    let mut guard = None;
    let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    for (y, line) in grid.iter().enumerate() {
        for (x, c) in line.iter().enumerate() {
            match c {
                '.' | '#' => {}
                &d => {
                    guard = Some(Guard {
                        pos: (x as isize, y as isize),
                        direction: Direction::from(d),
                    })
                }
            }
        }
    }

    let guard = guard.unwrap();
    let mut grid = Grid {
        height: grid.len(),
        width: grid[0].len(),
        inner: grid,
    };

    p1(&mut grid, guard);
    p2(grid, guard);
}

#[inline]
fn p1(grid: &mut Grid, mut guard: Guard) {
    while guard.step(grid) {
        grid.set(guard.pos, 'X');
    }

    // count
    let sum: usize = grid
        .inner
        .iter()
        .map(|r| r.iter().filter(|&&x| x == 'X').count())
        .sum();

    output(sum);
}

#[inline]
fn p2(grid: Grid, guard: Guard) {
    // there is probably a better algorithm but we'll just use brute force

    let sum: usize = (0..grid.width)
        .map(|x| {
            let mut g = grid.clone();
            (0..grid.height)
                .filter(|&y| loops_at(x, y, &mut g, &guard))
                .count()
        })
        .sum();

    output(sum);
}

/// Checks whether the guard loops if obstacle at (x,y).
fn loops_at(x: usize, y: usize, grid: &mut Grid, guard: &Guard) -> bool {
    let p = (x as isize, y as isize);

    // can't place on existing or guard
    if guard.pos == p || grid.obstacle(p) {
        return false;
    }

    // place and run
    let prev = grid.at(p);
    grid.set(p, '#');
    let looping = loops(grid, *guard);

    // reset grid
    grid.set(p, prev);

    looping
}

fn loops(grid: &Grid, mut guard: Guard) -> bool {
    let mut visited = HashSet::new();
    visited.insert(guard);

    while grid.inbound(guard.pos) {
        while guard.facing_obstacle(grid) {
            guard.direction.turn();
        }
        guard.pos = guard.direction.step(guard.pos);
        if !visited.insert(guard) {
            // looping
            return true;
        }
    }
    false
}
