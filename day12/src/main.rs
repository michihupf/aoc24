use std::collections::{HashMap, VecDeque};

use aoclib::{input, output};

#[derive(Debug, Clone, Copy)]
struct Garden {
    x: usize,
    y: usize,
    crop: char,
    explored: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum FenceDirection {
    North,
    West,
    South,
    East,
}

impl FenceDirection {
    fn vertical(&self) -> bool {
        matches!(self, Self::West | Self::East)
    }
}

#[derive(Debug, Clone, Copy)]
struct Fence {
    len: usize,
    direction: FenceDirection,
    x: usize,
    y: usize,
}

impl Fence {
    fn new(x: usize, y: usize, dir: FenceDirection, len: usize) -> Fence {
        Fence {
            x,
            y,
            direction: dir,
            len,
        }
    }
}

impl Fence {
    /// Returns true if `self` is right of `other`.
    fn right_of(&self, other: &Fence) -> bool {
        other.y == self.y && other.x == self.x.wrapping_sub(other.len)
    }

    /// Returns true if `self` is left of `other`.
    fn left_of(&self, other: &Fence) -> bool {
        other.right_of(self)
    }

    /// Returns true if `self` is below `other`.
    fn below(&self, other: &Fence) -> bool {
        other.x == self.x && other.y == self.y.wrapping_sub(other.len)
    }

    /// Returns true if `self` is above `other`.
    fn above(&self, other: &Fence) -> bool {
        other.below(self)
    }
}

#[derive(Debug)]
struct Region {
    /// map of gardens.
    gardens: Vec<Garden>,
    /// perimeter
    fences: HashMap<FenceDirection, Vec<Fence>>,
}

impl Region {
    /// Creates a new region.
    fn new() -> Self {
        let fence_map = HashMap::from([
            (FenceDirection::North, Vec::new()),
            (FenceDirection::West, Vec::new()),
            (FenceDirection::South, Vec::new()),
            (FenceDirection::East, Vec::new()),
        ]);
        Region {
            gardens: Vec::new(),
            fences: fence_map,
        }
    }

    /// Calculates the area.
    fn area(&self) -> usize {
        self.gardens.len()
    }

    /// Calculates the perimeter.
    fn perimeter(&self) -> usize {
        self.fences
            .values()
            .map(|fs| fs.iter().map(|f| f.len).sum::<usize>())
            .sum()
    }

    /// Calculates the number of sides.
    fn sides(&self) -> usize {
        self.fences.values().map(|fs| fs.len()).sum()
    }

    /// Adds a garden to the region.
    fn add_garden(&mut self, garden: Garden) {
        self.gardens.push(garden);
    }

    /// Add a fence on the position of provided garden.
    #[allow(non_snake_case)]
    fn add_fence(&mut self, mut f: Fence) {
        // look for adjacent fences with same direction.

        // an adjacent fence F to f will satisfy F.x == f.x - F.len and F.y == f.y if horizonal
        // or F.y == f.y - F.len and F.x == f.x if vertical
        // this only applies if f is right of/below F, so we also need to check the other way around to merge the other way

        let fences = self.fences.get_mut(&f.direction).unwrap();
        // left/above the fence
        let before;
        // right/below the fence
        let after;

        if f.direction.vertical() {
            before = fences.iter().position(|F| F.above(&f));
            after = fences.iter().position(|F| F.below(&f));
        } else {
            before = fences.iter().position(|F| F.left_of(&f));
            after = fences.iter().position(|F| F.right_of(&f));
        }

        match (before, after) {
            // new fence
            (None, None) => fences.push(f),
            // expand the one before
            (Some(b), None) => {
                fences[b].len += f.len;
            }
            // expand new fence by the one after and replace
            (None, Some(r)) => {
                f.len += fences[r].len;
                fences[r] = f;
            }
            // combination of case 2 and 3
            (Some(l), Some(r)) => {
                fences[l].len += f.len + fences[r].len;
                fences.remove(r);
            }
        }
    }
}

fn main() {
    let input = input("input");

    // parse gardens
    let mut gardens: Vec<Vec<Garden>> = input
        .lines()
        .enumerate()
        .map(|(y, l)| {
            l.chars()
                .enumerate()
                .map(move |(x, crop)| Garden {
                    x,
                    y,
                    crop,
                    explored: false,
                })
                .collect()
        })
        .collect();

    // use bfs to form regions
    let width = gardens[0].len();
    let height = gardens.len();
    let mut regions = Vec::new();
    for y in 0..gardens.len() {
        for x in 0..gardens[0].len() {
            if let Some(region) = find_region(gardens[y][x], &mut gardens, width, height) {
                regions.push(region);
            }
        }
    }

    p1(&regions);
    p2(&regions);
}

fn find_region(
    start: Garden,
    gardens: &mut [Vec<Garden>],
    width: usize,
    height: usize,
) -> Option<Region> {
    if start.explored {
        None
    } else {
        let mut region = Region::new();
        region.add_garden(start);
        let mut q = VecDeque::new();
        gardens[start.y][start.x].explored = true;
        q.push_back((start.x, start.y));
        while let Some(v) = q.pop_front() {
            // v is (x,y)
            for (x, y, dir) in [
                (v.0.wrapping_sub(1), v.1, FenceDirection::West),
                (v.0 + 1, v.1, FenceDirection::East),
                (v.0, v.1.wrapping_sub(1), FenceDirection::North),
                (v.0, v.1 + 1, FenceDirection::South),
            ] {
                if x < width && y < height {
                    if gardens[y][x].crop == gardens[v.1][v.0].crop {
                        if !gardens[y][x].explored {
                            gardens[y][x].explored = true;
                            region.add_garden(gardens[y][x]);
                            q.push_back((x, y));
                        }
                    } else {
                        region.add_fence(Fence::new(x, y, dir, 1));
                    }
                } else {
                    // map border
                    region.add_fence(Fence::new(x, y, dir, 1));
                }
            }
        }
        Some(region)
    }
}

#[inline]
fn p1(regions: &[Region]) {
    output(
        regions
            .iter()
            .map(|r| r.area() * r.perimeter())
            .sum::<usize>(),
    );
}

#[inline]
fn p2(regions: &[Region]) {
    output(regions.iter().map(|r| r.area() * r.sides()).sum::<usize>());
}
