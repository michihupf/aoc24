use std::collections::{BinaryHeap, HashMap};

use aoclib::{input, output, EAST, NORTH, SOUTH, WEST};

const SIMULATION_SIZE: usize = 1024;
const DIM: usize = 70;
type Vec2D = aoclib::Vec2D<i32>;

const START: Vec2D = Vec2D::new(0, 0);
const END: Vec2D = Vec2D::new(DIM as i32, DIM as i32);

#[derive(PartialEq, Eq)]
struct Node {
    cost: usize,
    pos: Vec2D,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.pos.cmp(&other.pos))
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

struct Grid {
    map: [[char; DIM + 1]; DIM + 1],
}

impl Grid {
    fn new() -> Self {
        Self {
            map: [['.'; DIM + 1]; DIM + 1],
        }
    }

    fn at(&self, pos: &Vec2D) -> char {
        assert!((0..=DIM).contains(&(pos.x as usize)));
        assert!((0..=DIM).contains(&(pos.y as usize)));
        self.map[pos.y as usize][pos.x as usize]
    }

    /// Returns the cost of the path or None if no path is found.
    fn dijkstra(&self) -> Option<usize> {
        let mut dist: HashMap<Vec2D, usize> = HashMap::new();
        // max heap but node ordering is swapped so it acts as min heap
        let mut heap = BinaryHeap::new();

        dist.insert(START, 0);
        heap.push(Node {
            cost: 0,
            pos: START,
        });

        while let Some(Node { cost, pos }) = heap.pop() {
            for dir in [NORTH, WEST, SOUTH, EAST] {
                let next = Node {
                    cost: cost + 1,
                    pos: pos + dir,
                };

                if next.pos == END {
                    return Some(next.cost);
                }

                if next.pos.x < 0
                    || next.pos.y < 0
                    || next.pos.x > DIM as i32
                    || next.pos.y > DIM as i32
                    || self.at(&next.pos) == '#'
                {
                    continue;
                }

                if let Some(d) = dist.get(&next.pos) {
                    if next.cost < *d {
                        dist.insert(next.pos, next.cost);
                        heap.push(next);
                    }
                } else {
                    dist.insert(next.pos, next.cost);
                    heap.push(next);
                }
            }
        }
        None
    }
}

fn main() {
    let input = input("input");

    let points: Vec<Vec2D> = input
        .lines()
        .map(|l| {
            let (l, r) = l.split_once(",").unwrap();
            Vec2D::new(l.parse().unwrap(), r.parse().unwrap())
        })
        .collect();

    p1(&points);
    p2(&points);
}

fn drop_bytes(size: usize, points: &[Vec2D]) -> Grid {
    let mut grid = Grid::new();
    points.iter().take(size).for_each(|p| {
        grid.map[p.y as usize][p.x as usize] = '#';
    });
    grid
}

#[inline]
fn p1(points: &[Vec2D]) {
    let grid = drop_bytes(SIMULATION_SIZE, points);
    output(grid.dijkstra().unwrap());
}

#[inline]
fn p2(points: &[Vec2D]) {
    let mut grid = drop_bytes(SIMULATION_SIZE, points);
    for byte in points.iter().skip(SIMULATION_SIZE) {
        // drop in the byte
        grid.map[byte.y as usize][byte.x as usize] = '#';
        // test if works
        if grid.dijkstra().is_none() {
            output(format!("{},{}", byte.x, byte.y));
            break;
        }
    }

    // output(result);
}
