use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap, HashSet},
};

use aoclib::{input, output};

type Vec2D = aoclib::Vec2D<i32>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    North = 0,
    West = 1,
    South = 2,
    East = 3,
}

impl Direction {
    /// Returns the number of 90deg turns required to face ot other from self.
    fn turns_to(&self, other: Direction) -> i32 {
        // diff between enum indeces will be number of turns. Wrap 3 to 1.
        match (*self as i32 - other as i32).abs() {
            3 => 1,
            n => n,
        }
    }

    fn all() -> impl Iterator<Item = &'static Direction> {
        [
            Direction::North,
            Direction::West,
            Direction::South,
            Direction::East,
        ]
        .iter()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Node {
    cost: i32,
    pos: Vec2D,
    facing: Direction,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Flip ordering of costs to make a min-heap
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

struct Maze {
    start: Vec2D,
    end: Vec2D,
    walls: HashSet<Vec2D>,
    best_cost: Option<i32>,
    unique_tiles: Option<usize>,
}

type VecDir = (Vec2D, Direction);
impl Maze {
    /// Creates a new maze.
    fn new(start: Vec2D, end: Vec2D, walls: HashSet<Vec2D>) -> Self {
        Self {
            start,
            end,
            walls,
            best_cost: None,
            unique_tiles: None,
        }
    }

    /// Solves the maze.
    fn solve(&mut self) {
        // use dijkstra on weighted path graph
        // edges are always every direction where the next node is not a wall
        // cost is dependant on direction

        let mut dist: HashMap<VecDir, i32> = HashMap::new();
        let mut heap = BinaryHeap::new();
        // set of tiles on shortest paths
        let mut parents: HashMap<VecDir, HashSet<VecDir>> = HashMap::new();

        dist.insert((self.start, Direction::East), 0);
        heap.push(Node {
            cost: 0,
            pos: self.start,
            facing: Direction::East,
        });

        while let Some(Node { cost, pos, facing }) = heap.pop() {
            // now check every direction
            for (dp, dd) in [
                (Vec2D::new(0, -1), Direction::North),
                (Vec2D::new(-1, 0), Direction::West),
                (Vec2D::new(0, 1), Direction::South),
                (Vec2D::new(1, 0), Direction::East),
            ] {
                let turns = facing.turns_to(dd);
                if turns == 2 {
                    // turning a 180 will make the path longer
                    continue;
                }

                let next = Node {
                    cost: cost + turns * 1000 + 1,
                    pos: pos + dp,
                    facing: dd,
                };

                if self.walls.contains(&next.pos) {
                    // hit a wall
                    continue;
                }

                let key = (next.pos, next.facing);
                if let Some(d) = dist.get(&key) {
                    match next.cost.cmp(d) {
                        Ordering::Less => {
                            heap.push(next);
                            // found better way
                            dist.insert(key, next.cost);
                            parents.insert(key, HashSet::from([(pos, facing)]));
                        }
                        Ordering::Equal => {
                            heap.push(next);
                            parents.get_mut(&key).unwrap().insert((pos, facing));
                        }
                        _ => {}
                    }
                } else {
                    heap.push(next);
                    dist.insert(key, next.cost);
                    parents.insert(key, HashSet::from([(pos, facing)]));
                }
            }
        }

        for &d in Direction::all() {
            match self.best_cost {
                None => self.best_cost = dist.get(&(self.end, d)).copied(),
                Some(v) => {
                    if *dist.get(&(self.end, d)).unwrap_or(&i32::MAX) < v {
                        self.best_cost = Some(*dist.get(&(self.end, d)).unwrap_or(&i32::MAX))
                    }
                }
            }
        }
        if self.best_cost.is_none() {
            // Maze is not solvable.
            return;
        }

        let mut visited = HashSet::new();
        for &d in Direction::all() {
            if let Some(&cost) = dist.get(&(self.end, d)) {
                if cost == self.best_cost.unwrap() {
                    visited.extend(Maze::tiles_to((self.end, d), &parents));
                }
            }
        }
        let visited: HashSet<_> = visited.iter().map(|(pos, _)| pos).collect();
        self.unique_tiles = Some(visited.len());
    }

    /// Returns a set of all tiles visted to reach e.
    fn tiles_to(e: VecDir, parents: &HashMap<VecDir, HashSet<VecDir>>) -> HashSet<VecDir> {
        let mut set = HashSet::new();
        set.insert(e);
        if let Some(ps) = parents.get(&e) {
            for parent in ps {
                set.extend(Maze::tiles_to(*parent, parents));
            }
        }

        set
    }
}

fn main() {
    let input = input("input");

    let mut start = Vec2D::new(1, 1);
    let mut end = Vec2D::new(1, 1);
    let mut walls = HashSet::new();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            match c {
                '#' => {
                    walls.insert(Vec2D::new(x as i32, y as i32));
                }
                'S' => {
                    start = Vec2D::new(x as i32, y as i32);
                }
                'E' => {
                    end = Vec2D::new(x as i32, y as i32);
                }
                _ => {}
            }
        }
    }

    let mut maze = Maze::new(start, end, walls);

    maze.solve();

    p1(&maze);
    p2(&maze);
}

#[inline]
fn p1(maze: &Maze) {
    output(maze.best_cost.expect("Maze not solved."));
}

#[inline]
fn p2(maze: &Maze) {
    output(maze.unique_tiles.expect("Maze not solved"));
}
