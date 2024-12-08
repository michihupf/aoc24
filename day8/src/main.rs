use std::collections::HashSet;

use aoclib::{input, output};

type Pos = (isize, isize);

#[derive(Debug)]
struct Antenna {
    pub pos: Pos,
    pub freq: char,
}

fn main() {
    let input = input("input");

    let antennas: Vec<Antenna> = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, c)| *c != '.')
                .map(move |(x, freq)| Antenna {
                    pos: (x as isize, y as isize),
                    freq,
                })
        })
        .collect();

    let width = input.lines().next().unwrap().len() as isize;
    let height = input.lines().count() as isize;

    p1(&antennas, width, height);
    p2(&antennas, width, height);
}

fn calc_nodes(antennas: &[Antenna], width: isize, height: isize, resharm: bool) -> usize {
    let mut antinodes = HashSet::new();
    for (i, t1) in antennas.iter().enumerate() {
        for t2 in antennas[i + 1..].iter() {
            if t1.freq != t2.freq {
                continue;
            }

            // t1 and t2 are on a line y=mx+b,
            // so we can just step forward and backwards with differences
            let dx = t2.pos.0 - t1.pos.0;
            let dy = t2.pos.1 - t1.pos.1;

            let (dx, dy) = if t1.pos.0 + dx == t2.pos.0 && t1.pos.1 + dy == t2.pos.1 {
                (-dx, -dy)
            } else {
                (dx, dy)
            };

            let mut p1: Pos = (t1.pos.0 + dx, t1.pos.1 + dy);
            let mut p2: Pos = (t2.pos.0 - dx, t2.pos.1 - dy);

            if inbound(p1, width, height) {
                antinodes.insert(p1);
            }
            if inbound(p2, width, height) {
                antinodes.insert(p2);
            }

            // part 2
            if resharm {
                while inbound(p1, width, height) {
                    antinodes.insert(p1);
                    p1 = (p1.0 + dx, p1.1 + dy);
                }

                while inbound(p2, width, height) {
                    antinodes.insert(p2);
                    p2 = (p2.0 - dx, p2.1 - dy);
                }

                antinodes.insert(t1.pos);
                antinodes.insert(t2.pos);
            }
        }
    }

    antinodes.len()
}

#[inline]
fn p1(antennas: &[Antenna], width: isize, height: isize) {
    output(calc_nodes(antennas, width, height, false));
}

fn inbound(p: Pos, width: isize, height: isize) -> bool {
    p.0 >= 0 && p.0 < width && p.1 >= 0 && p.1 < height
}

#[inline]
fn p2(antennas: &[Antenna], width: isize, height: isize) {
    output(calc_nodes(antennas, width, height, true));
}
