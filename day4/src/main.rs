use std::slice::Iter;

use aoclib::{input, output};

fn main() {
    let input = input("example2");

    let vec: Vec<&[u8]> = input.lines().map(|l| l.as_bytes()).collect();

    p1(&vec);
    p2(&vec);
}

#[derive(Debug)]
enum Direction {
    North,
    NorthWest,
    West,
    SouthWest,
    South,
    SouthEast,
    East,
    NorthEast,
}

impl Direction {
    /// Returns (m * dy, m * dx).
    fn d(&self, m: i32) -> (i32, i32) {
        match self {
            Direction::North => (-m, 0),
            Direction::NorthWest => (-m, -m),
            Direction::West => (0, -m),
            Direction::SouthWest => (m, -m),
            Direction::South => (m, 0),
            Direction::SouthEast => (m, m),
            Direction::East => (0, m),
            Direction::NorthEast => (-m, m),
        }
    }

    fn iter() -> Iter<'static, Direction> {
        use Direction::*;
        static DIRECTIONS: [Direction; 8] = [
            North, NorthWest, West, SouthWest, South, SouthEast, East, NorthEast,
        ];
        DIRECTIONS.iter()
    }
}

const XMAS: [u8; 4] = [b'X', b'M', b'A', b'S'];

#[inline]
fn p1(input: &[&[u8]]) {
    let height = input.len() as i32;
    let width = input[0].len() as i32;

    let mut count = 0;
    // time complexity O(width * height)
    for y in 0..height {
        for x in 0..width {
            // look for xmas in every direction
            'dirl: for dir in Direction::iter() {
                for i in 0..4 {
                    let v = dir.d(i);
                    let (y, x) = (y + v.0, x + v.1);
                    if y >= height || y < 0 || x >= width || x < 0 {
                        continue 'dirl;
                    }
                    if input[y as usize][x as usize] != XMAS[i as usize] {
                        continue 'dirl;
                    }
                }
                // found XMAS
                count += 1;
            }
        }
    }

    output(count);
}

#[inline]
fn p2(input: &[&[u8]]) {
    let height = input.len();
    let width = input[0].len();

    let mut count = 0;
    for y in 1..height - 1 {
        for x in 1..width - 1 {
            // look for an X-MAS. A is always in center.
            if input[y][x] != b'A' {
                continue;
            }
            // letter counts
            let mut ms = 0;
            let mut ss = 0;

            for i in [-1, 1] {
                for j in [-1, 1] {
                    match input[(y as i32 + i) as usize][(x as i32 + j) as usize] {
                        b'M' => ms += 1,
                        b'S' => ss += 1,
                        _ => {}
                    }
                }
            }

            // we want 2 M and 2 S and the diagonal to be different
            if ms == 2 && ss == 2 && input[y - 1][x - 1] != input[y + 1][x + 1] {
                count += 1;
            }
        }
    }

    output(count);
}
