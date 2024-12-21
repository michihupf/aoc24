use std::collections::{BinaryHeap, HashMap, HashSet};

use aoclib::{input, output, CARDINAL};

type Vec2D = aoclib::Vec2D<i32>;

struct Grid {
    start: Vec2D,
    end: Vec2D,
    walls: HashSet<Vec2D>,
}

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

impl Grid {
    fn new(start: Vec2D, end: Vec2D, walls: HashSet<Vec2D>) -> Grid {
        Grid { start, end, walls }
    }

    /// Computes the by definition unique path between start and end.
    fn dijkstra(&self) -> Vec<Vec2D> {
        let mut dist: HashMap<Vec2D, usize> = HashMap::new();
        // max heap but node ordering reversed
        let mut heap = BinaryHeap::new();
        let mut parents: HashMap<Vec2D, Vec2D> = HashMap::new();

        dist.insert(self.start, 0);
        heap.push(Node {
            cost: 0,
            pos: self.start,
        });

        while let Some(Node { cost, pos }) = heap.pop() {
            for dir in CARDINAL {
                let next = Node {
                    cost: cost + 1,
                    pos: pos + dir,
                };

                if next.pos == self.end {
                    // done, unwind parent
                    let mut v = Grid::unwind_parents(pos, &parents);
                    v.push(self.end);
                    return v;
                }

                if self.walls.contains(&next.pos) {
                    // WE WILL NEVER CHEAT. We want to find the intended route.
                    continue;
                }

                if let Some(&d) = dist.get(&next.pos) {
                    if next.cost < d {
                        dist.insert(next.pos, next.cost);
                        parents.insert(next.pos, pos);
                        heap.push(next);
                    }
                } else {
                    dist.insert(next.pos, next.cost);
                    parents.insert(next.pos, pos);
                    heap.push(next);
                }
            }
        }

        vec![]
    }

    fn unwind_parents(node: Vec2D, parents: &HashMap<Vec2D, Vec2D>) -> Vec<Vec2D> {
        if let Some(&parent) = parents.get(&node) {
            let mut v = Grid::unwind_parents(parent, parents);
            v.push(node);
            v
        } else {
            vec![node]
        }
    }
}

fn main() {
    let input = input("input");

    let mut start = Vec2D::new(0, 0);
    let mut end = Vec2D::new(1, 1);
    let mut walls = HashSet::new();

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            match c {
                '#' => {
                    walls.insert(Vec2D::new(x as i32, y as i32));
                }
                'S' => start = Vec2D::new(x as i32, y as i32),
                'E' => end = Vec2D::new(x as i32, y as i32),
                _ => {}
            }
        }
    }

    let grid = Grid::new(start, end, walls);
    let path = grid.dijkstra();

    p1(&path);
    p2(&path);
}

/// Returns the number of cheat paths that save `timesave` or more time.
fn count_2ps_cheat_paths(path: &[Vec2D], timesave: usize) -> usize {
    let times: HashMap<Vec2D, usize> = path.iter().enumerate().map(|(x, &y)| (y, x)).collect();
    let mut count = 0;
    // for every point check every cheat
    for &p in path {
        // O(n)
        let t = times[&p];
        // check step 1 in all cardinal directions
        for d in CARDINAL {
            let np = p + d;
            // now check every follow-up-step
            for d2 in CARDINAL {
                let nnp = np + d2;
                if let Some(&nnt) = times.get(&nnp) {
                    if nnt >= t + 2 && nnt - (t + 2) >= timesave {
                        count += 1;
                    }
                }
            }
        }
    }
    count
}

#[inline]
fn p1(path: &[Vec2D]) {
    output(count_2ps_cheat_paths(path, 100));
}

#[inline]
fn p2(path: &[Vec2D]) {
    let mut count = 0;
    let intended_length = path.len();
    // for every pair a cheat needs to have l1 distance <= 20
    for (i, &p) in path.iter().enumerate() {
        for (j, &other) in path.iter().enumerate().skip(i + 1) {
            // way left (subtract start and intermediate path)
            let way_to_go = intended_length - j;
            let traveled = p.x.abs_diff(other.x) + p.y.abs_diff(other.y);
            if traveled <= 20 && intended_length - way_to_go - traveled as usize - i >= 100 {
                count += 1;
            }
        }
    }
    output(count);
}
