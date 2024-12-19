use std::collections::HashMap;

use aoclib::{input, output};

fn main() {
    let input = input("input");

    let mut lines = input.lines();
    let patterns: Vec<&str> = lines.next().unwrap().split(", ").collect();
    lines.next().unwrap(); // empty line
    let towels: Vec<&str> = lines.collect();

    p1(&towels, &patterns);
    p2(&towels, &patterns);
}

/// Greedily search for a solution.
fn backtrack(towel: &str, patterns: &[&str], memo: &mut HashMap<String, bool>) -> bool {
    if towel.is_empty() {
        return true;
    }

    if let Some(&b) = memo.get(towel) {
        return b;
    }

    for pattern in patterns {
        if towel.starts_with(pattern) && backtrack(&towel[pattern.len()..], patterns, memo) {
            memo.insert(towel.to_owned(), true);
            return true;
        }
    }

    memo.insert(towel.to_owned(), false);
    false
}

/// Searches all solutions, slower.
fn backtrack_count(towel: &str, patterns: &[&str], memo: &mut HashMap<String, u64>) -> u64 {
    if towel.is_empty() {
        return 1;
    }

    if let Some(&num) = memo.get(towel) {
        return num;
    }

    let mut count = 0;
    for pattern in patterns {
        if let Some(stripped) = towel.strip_prefix(pattern) {
            count += backtrack_count(stripped, patterns, memo)
        }
    }

    memo.insert(towel.to_owned(), count);
    count
}

#[inline]
fn p1(towels: &[&str], patterns: &[&str]) {
    let mut memo = HashMap::new();
    let possible = towels
        .iter()
        .filter_map(|t| backtrack(t, patterns, &mut memo).then_some(true))
        .count();
    output(possible);
}

#[inline]
fn p2(towels: &[&str], patterns: &[&str]) {
    let mut memo = HashMap::new();
    let all: u64 = towels
        .iter()
        .map(|t| backtrack_count(t, patterns, &mut memo))
        .sum();
    output(all);
}
