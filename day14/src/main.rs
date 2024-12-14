use aoclib::{input, output, Vec2D};

const WIDTH: isize = 101; // example: 11, input: 101
const HEIGHT: isize = 103; // example:  7, input: 103

#[derive(Debug, Clone)]
struct Robot {
    p: Vec2D<isize>,
    v: Vec2D<isize>,
}

fn main() {
    let input = input("input");

    let robots: Vec<Robot> = input
        .lines()
        .map(|l| {
            let (p, v) = l.split_once(" ").unwrap();
            let p = p
                .chars()
                .skip(2)
                .collect::<String>()
                .split_once(",")
                .map(|(l, r)| Vec2D::<isize>::new(l.parse().unwrap(), r.parse().unwrap()))
                .unwrap();
            let v = v
                .chars()
                .skip(2)
                .collect::<String>()
                .split_once(",")
                .map(|(l, r)| Vec2D::<isize>::new(l.parse().unwrap(), r.parse().unwrap()))
                .unwrap();
            Robot { p, v }
        })
        .collect();

    p1(robots.clone());
    p2(robots);
}

fn p1(mut robots: Vec<Robot>) {
    use std::cmp::Ordering::{Equal, Greater, Less};
    // postion of a bot after 100 seconds is p+100v mod WIDTH

    robots.iter_mut().for_each(|robot| {
        robot.p = (robot.p + (robot.v * 100))
            % Vec2D {
                x: WIDTH,
                y: HEIGHT,
            };
    });

    let mut q1 = 0;
    let mut q2 = 0;
    let mut q3 = 0;
    let mut q4 = 0;
    for r in robots {
        match r.p.x.cmp(&(WIDTH / 2)) {
            Less => match r.p.y.cmp(&(HEIGHT / 2)) {
                Less => q2 += 1,
                Greater => q3 += 1,
                Equal => {}
            },
            Greater => match r.p.y.cmp(&(HEIGHT / 2)) {
                Less => q1 += 1,
                Greater => q4 += 1,
                Equal => {}
            },
            Equal => {}
        }
    }

    let safety_factor = q1 * q2 * q3 * q4;

    output(safety_factor);
}

fn modulus(num: isize, _mod: isize) -> isize {
    ((num % _mod) + _mod) % _mod
}

fn p2(mut robots: Vec<Robot>) {
    // look at variances in X and Y,
    // when we have a christmas tree the variance should drop dramatically
    // because we have a huge bot cluster.

    // let mut backup = robots.clone();

    // pick the lowest variances for X
    let mut lowest = f32::MAX;
    let mut bx = 0;
    for t in 1..=WIDTH {
        robots
            .iter_mut()
            .for_each(|r| r.p.x = modulus(r.p.x + r.v.x, WIDTH));

        // calculate the variance in X
        let mean = robots.iter().map(|r| r.p.x as f32).sum::<f32>() / robots.len() as f32;
        let var = robots
            .iter()
            .map(|r| (r.p.x as f32 - mean) * (r.p.x as f32 - mean))
            .sum::<f32>()
            / robots.len() as f32;

        if var < lowest {
            lowest = var;
            bx = t;
        }
    }

    // do the same for Y
    let mut lowest = f32::MAX;
    let mut by = 0;
    for t in 1..=HEIGHT {
        robots
            .iter_mut()
            .for_each(|r| r.p.y = modulus(r.p.y + r.v.y, HEIGHT));

        // calculate the variance in Y
        let mean = robots.iter().map(|r| r.p.y as f32).sum::<f32>() / robots.len() as f32;
        let var = robots
            .iter()
            .map(|r| (r.p.y as f32 - mean) * (r.p.y as f32 - mean))
            .sum::<f32>()
            / robots.len() as f32;

        if var < lowest {
            lowest = var;
            by = t;
        }
    }
    // as we are in modular arithmetic these variance dips align after k repeats
    // so t = bx + kW and t = by (mod H)

    // ==> by = bx + kW (mod H) ==> k = (by - bx) * inv(W) (mod H); where inv(W) is the modular multiplicative inverse

    // Bezouts identity: Wa+Hb=gcd(W,H)=1 so multiplicative inverse of W mod H is a

    let (_, inv_w, _) = extended_gcd(WIDTH, HEIGHT);
    let t = bx + modulus(inv_w * (by - bx), HEIGHT) * WIDTH;

    // backup.iter_mut().for_each(|robot| {
    //     robot.p = (robot.p + (robot.v * t))
    //         % Vec2D {
    //             x: WIDTH,
    //             y: HEIGHT,
    //         };
    // });
    // // plot the found grid
    // for y in 0..HEIGHT {
    //     for x in 0..WIDTH {
    //         let bots = if backup.iter().filter(|&r| r.p.x == x && r.p.y == y).count() >= 1 {
    //             'X'
    //         } else {
    //             '-'
    //         };
    //         print!("{bots}")
    //     }
    //     println!();
    // }

    output(t);
}

/// Returns (gcd, x, y) for ax+by=gcd(a,b).
const fn extended_gcd(a: isize, b: isize) -> (isize, isize, isize) {
    let (mut old_r, mut r) = (a, b);
    let (mut old_s, mut s) = (1, 0);
    let (mut old_t, mut t) = (0, 1);

    while r != 0 {
        let q = old_r / r;
        (old_r, r) = (r, old_r - q * r);
        (old_s, s) = (s, old_s - q * s);
        (old_t, t) = (t, old_t - q * t);
    }

    (old_r, old_s, old_t)
}
