use std::collections::VecDeque;

use aoclib::{input, output};

#[derive(Debug, Clone, Copy)]
struct Node {
    pub x: usize,
    pub y: usize,
    pub level: i8,
    pub explored: bool,
}

fn main() {
    let input = input("input");

    let map: Vec<Vec<Node>> = input
        .lines()
        .enumerate()
        .map(|(y, l)| {
            l.chars()
                .enumerate()
                .map(|(x, c)| Node {
                    x,
                    y,
                    level: c.to_string().parse::<i8>().unwrap(),
                    explored: false,
                })
                .collect()
        })
        .collect();

    p1(&map);
    p2(&map);
}

#[inline]
fn p1(map: &[Vec<Node>]) {
    output(calculate_scores(map, false).iter().sum::<i32>());
}

#[inline]
fn p2(map: &[Vec<Node>]) {
    output(calculate_scores(map, true).iter().sum::<i32>());
}

/// Calculates the scores for all heads in reading order. If `rating` is true then calculates ratings instead.
fn calculate_scores(map: &[Vec<Node>], rating: bool) -> Vec<i32> {
    let heads: Vec<(usize, usize)> = map
        .iter()
        .flat_map(|row| {
            row.iter()
                .filter_map(|n| if n.level == 0 { Some((n.x, n.y)) } else { None })
                .collect::<Vec<(usize, usize)>>()
        })
        .collect();

    let mut scores = vec![0; heads.len()];
    let width = map[0].len();
    let height = map.len();

    // from each head search trails using bfs for all trails - they are all equal length
    heads.iter().enumerate().for_each(|(i, pos)| {
        let mut m = map.to_owned();
        let mut q = VecDeque::new();
        m[pos.1][pos.0].explored = true;
        q.push_back(m[pos.1][pos.0]);
        while !q.is_empty() {
            let v = q.pop_front().unwrap();
            if v.level == 9 {
                scores[i] += 1;
            } else {
                // search every direction:
                // left
                if v.x > 0 {
                    let left = &mut m[v.y][v.x - 1];
                    if !left.explored && left.level == v.level + 1 {
                        if !rating {
                            left.explored = true;
                        }
                        q.push_back(*left);
                    }
                }
                // right
                if v.x < width - 1 {
                    let right = &mut m[v.y][v.x + 1];
                    if !right.explored && right.level == v.level + 1 {
                        if !rating {
                            right.explored = true;
                        }
                        q.push_back(*right);
                    }
                }
                // up
                if v.y > 0 {
                    let up = &mut m[v.y - 1][v.x];
                    if !up.explored && up.level == v.level + 1 {
                        if !rating {
                            up.explored = true;
                        }
                        q.push_back(*up);
                    }
                }
                // down
                if v.y < height - 1 {
                    let down = &mut m[v.y + 1][v.x];
                    if !down.explored && down.level == v.level + 1 {
                        if !rating {
                            down.explored = true;
                        }
                        q.push_back(*down);
                    }
                }
            }
        }
    });

    scores
}
