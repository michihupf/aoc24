use std::collections::{HashMap, HashSet};

use aoclib::{input, output};

const MOD: i64 = 16777216;

fn main() {
    let input = input("input");

    let init: Vec<_> = input.lines().flat_map(|l| l.parse::<i64>()).collect();
    // let init = vec![123];

    p1(init.clone());
    p2(init);
}

#[inline]
fn p1(mut init: Vec<i64>) {
    init.iter_mut().for_each(|s| {
        for _ in 0..2000 {
            // step 1
            *s ^= *s * 64;
            *s %= MOD;
            // step 2
            *s ^= *s / 32;
            *s %= MOD;
            // step 3
            *s ^= *s * 2048;
            *s %= MOD;
        }
    });

    output(init.iter().sum::<i64>());
}

#[inline]
fn p2(mut init: Vec<i64>) {
    let mut seqs: Vec<Vec<i64>> = Vec::with_capacity(init.len());
    init.iter_mut().for_each(|s| {
        let mut prices = Vec::with_capacity(2001);
        prices.push(*s % 10);
        for _ in 0..2000 {
            // step 1
            *s ^= *s * 64;
            *s %= MOD;
            // step 2
            *s ^= *s / 32;
            *s %= MOD;
            // step 3
            *s ^= *s * 2048;
            *s %= MOD;
            prices.push(*s % 10);
        }
        seqs.push(prices);
    });

    // map seq pattern to accumulated price
    let mut map = HashMap::new();

    // check every diff pattern and accumulate
    seqs.into_iter().for_each(|s| {
        let diffs: Vec<i64> = s.windows(2).map(|w| w[1] - w[0]).collect();
        let mut seen = HashSet::new();
        for i in 0..diffs.len() - 4 {
            // 4 diff seq
            let seq: [i64; 4] = diffs[i..i + 4].try_into().unwrap();
            if !seen.contains(&seq) {
                // add the price to the entry
                *map.entry(seq).or_insert(0) += s[i + 4];
                seen.insert(seq);
            }
        }
    });

    output(map.values().max().unwrap());
}
