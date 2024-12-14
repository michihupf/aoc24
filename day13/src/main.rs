use std::ops::Add;

use aoclib::{input, output};

#[derive(Debug, Clone, Copy)]
struct Vector {
    x: i64,
    y: i64,
}

impl Add for Vector {
    type Output = Vector;

    fn add(self, rhs: Self) -> Self::Output {
        Vector {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Vector {
    fn new() -> Vector {
        Vector { x: 0, y: 0 }
    }

    fn set_from_line(&mut self, line: &str, delim: &str) {
        let (x, y) = line.split_once(":").unwrap().1.split_once(", ").unwrap();
        self.x = x.split_once(delim).unwrap().1.parse().unwrap();
        self.y = y.split_once(delim).unwrap().1.parse().unwrap();
    }
}

#[derive(Debug)]
struct Machine {
    a: Vector,
    b: Vector,
    prize: Vector,
}

fn main() {
    let input = input("input");

    let mut machines = Vec::new();
    let mut a = Vector::new();
    let mut b = Vector::new();
    let mut prize = Vector::new();
    for l in input.lines() {
        if l.starts_with("Button A:") {
            a.set_from_line(l, "+");
        } else if l.starts_with("Button B:") {
            b.set_from_line(l, "+");
        } else if l.starts_with("Prize:") {
            prize.set_from_line(l, "=");
            machines.push(Machine { a, b, prize })
        }
    }

    p1(&machines);
    p2(&machines);
}

#[inline]
fn p1(machines: &[Machine]) {
    output(
        machines
            .iter()
            .map(|machine| {
                let a = machine.a;
                let b = machine.b;
                let x = machine.prize;

                let w = (x.x * a.y - x.y * a.x) / (b.x * a.y - b.y * a.x);
                let v = (x.y - w * b.y) / a.y;

                if v * a.x + w * b.x != x.x || v * a.y + w * b.y != x.y {
                    return 0;
                }

                if w >= 0 && v >= 0 && w <= 100 && v <= 100 {
                    3 * v + w
                } else {
                    0
                }
            })
            .sum::<i64>(),
    )
}

#[inline]
fn p2(machines: &[Machine]) {
    output(
        machines
            .iter()
            .map(|machine| {
                let a = machine.a;
                let b = machine.b;
                let x = machine.prize
                    + Vector {
                        x: 10000000000000,
                        y: 10000000000000,
                    };

                let w = (x.x * a.y - x.y * a.x) / (b.x * a.y - b.y * a.x);
                let v = (x.y - w * b.y) / a.y;

                if v * a.x + w * b.x != x.x || v * a.y + w * b.y != x.y {
                    return 0;
                }

                if w >= 0 && v >= 0 {
                    3 * v + w
                } else {
                    0
                }
            })
            .sum::<i64>(),
    )
}
