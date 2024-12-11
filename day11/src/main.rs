use std::{collections::HashMap, iter::successors};

use aoclib::{input, output};

type Stone = u64;

fn num_digits(num: u64) -> u32 {
    successors(Some(1), |&x| (x <= num).then(|| x * 10)).count() as u32 - 1
}

fn main() {
    let input = input("input");

    let stones: Vec<Stone> = input
        .split_whitespace()
        .map(|x| x.parse::<Stone>().unwrap())
        .collect();

    p1(&stones);
    p2(&stones);
}

fn blink(stone: Stone, blinks: i32, cache: &mut HashMap<(Stone, i32), u64>) -> u64 {
    if blinks == 0 {
        return 1;
    }
    if let Some(num_stones) = cache.get(&(stone, blinks)) {
        // same number of blinks and same stone value means everything will be the same from now on
        return *num_stones;
    }

    let num_after = {
        if stone == 0 {
            blink(1, blinks - 1, cache)
        } else {
            let digits = num_digits(stone);
            if digits % 2 == 0 {
                let middle = 10u64.pow(digits / 2);
                let (l, r) = (stone / middle, stone % middle);
                blink(l, blinks - 1, cache) + blink(r, blinks - 1, cache)
            } else {
                blink(stone * 2024, blinks - 1, cache)
            }
        }
    };
    // memoize
    cache.insert((stone, blinks), num_after);
    num_after
}

#[inline]
fn p1(stones: &[Stone]) {
    let mut cache = HashMap::new();
    output(
        stones
            .iter()
            .map(|&stone| blink(stone, 25, &mut cache))
            .sum::<u64>(),
    );
}

#[inline]
fn p2(stones: &[Stone]) {
    let mut cache = HashMap::new();
    output(
        stones
            .iter()
            .map(|&stone| blink(stone, 75, &mut cache))
            .sum::<u64>(),
    )
}
