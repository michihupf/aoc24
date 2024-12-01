use aoclib::{input, output};

fn main() {
    let input = input("input");

    let num_lines = input.lines().count();
    let mut left = Vec::with_capacity(num_lines);
    let mut right = Vec::with_capacity(num_lines);
    for line in input.lines() {
        let (l, r) = line.split_once("   ").unwrap();
        let l: u32 = l.parse().unwrap();
        let r: u32 = r.parse().unwrap();

        left.push(l);
        right.push(r);
    }

    p1(&mut left, &mut right);
    // vecs are now sorted
    p2(left, right);
}

#[inline]
fn p1(left: &mut [u32], right: &mut [u32]) {
    // Sorting does not need to be stable - relative order does not matter.
    left.sort_unstable();
    right.sort_unstable();

    let result: u32 = left
        .iter()
        .zip(right.iter())
        .map(|(&l, &r)| l.abs_diff(r))
        .sum();

    output(result);
}

#[inline]
fn p2(left: Vec<u32>, right: Vec<u32>) {
    // as the list are sorted we can traverse the list linearly and not miss any number.
    // we start all the way at the start.
    let mut r_idx = 0;
    let mut sum = 0;
    let mut prev_l = 0;
    let mut count = 0;

    for l in left {
        // update counter if l != prev_l
        if l != prev_l {
            count = 0;
            while r_idx < right.len() && l >= right[r_idx] {
                if l == right[r_idx] {
                    count += 1;
                }
                r_idx += 1;
            }
        }
        sum += l * count;
        prev_l = l;
    }

    output(sum);
}
