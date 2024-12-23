use std::collections::HashSet;

use aoclib::{input, output};

fn main() {
    let input = input("input");

    // build a graph
    let mut nodes = HashSet::new();
    let mut edges = HashSet::new();
    input.lines().for_each(|c| {
        let (l, r) = c.split_once("-").unwrap();
        nodes.insert(l);
        nodes.insert(r);
        edges.insert((l, r));
        edges.insert((r, l));
    });

    let nodes: Vec<&str> = nodes.into_iter().collect();
    let edges = edges
        .into_iter()
        .map(|(l, r)| {
            let lidx = nodes.iter().position(|&x| x == l).unwrap();
            let ridx = nodes.iter().position(|&x| x == r).unwrap();
            (lidx, ridx)
        })
        .collect();

    p1(&nodes, &edges);
    p2(&nodes, &edges);
}

/// Use dfs to find a cycle of `n` computers beginning from `vert` and ending at `goal`.
fn find_cycle_from(
    vert: usize,
    goal: usize,
    n: i32,
    nodes: &[&str],
    edges: &HashSet<(usize, usize)>,
    seen: &mut [bool],
    mut tcomp: bool, // true when we found the chief historians computer
) -> usize {
    if nodes[vert].starts_with('t') {
        tcomp = true;
    }
    // closing edge?
    if n == 1 {
        // if vert is adjacent to start we found a cycle
        if tcomp && edges.contains(&(vert, goal)) {
            return 1;
        } else {
            return 0;
        }
    }

    seen[vert] = true;
    let mut count = 0;
    for i in 0..nodes.len() {
        // can we travel to i? yes? then find a cycle from there, if seen we don't revisit
        if !seen[i] && edges.contains(&(vert, i)) {
            count += find_cycle_from(i, goal, n - 1, nodes, edges, seen, tcomp);
        }
    }
    seen[vert] = false;
    count
}

#[inline]
fn p1(nodes: &[&str], edges: &HashSet<(usize, usize)>) {
    let mut seen = vec![false; nodes.len()];

    let mut count = 0;
    for i in 0..nodes.len() - 2 {
        count += find_cycle_from(i, i, 3, nodes, edges, &mut seen.clone(), false);

        seen[i] = true;
    }

    // we counted clockwise and counterclockwise
    count /= 2;
    output(count);
}

fn neighbors(node: usize, edges: &HashSet<(usize, usize)>) -> HashSet<usize> {
    edges
        .iter()
        .filter(|(l, _)| *l == node)
        .map(|(_, r)| *r)
        .collect()
}

fn bron_kerbosch(
    r: HashSet<usize>,
    p: HashSet<usize>,
    x: HashSet<usize>,
    edges: &HashSet<(usize, usize)>,
) -> HashSet<usize> {
    if p.is_empty() && x.is_empty() {
        return r;
    }

    let mut best = HashSet::new();

    // get first element of p (or x if p was empty) as pivot
    let pivot = *p.iter().next().unwrap_or_else(|| x.iter().next().unwrap());
    let pivot_neigh = neighbors(pivot, edges);

    for &v in p.difference(&pivot_neigh) {
        let neigh = neighbors(v, edges);
        // recursive call on R u {v}, P n N(v) and X n N(v)
        let cq = bron_kerbosch(
            r.union(&HashSet::from([v])).copied().collect(),
            p.intersection(&neigh).copied().collect(),
            x.intersection(&neigh).copied().collect(),
            edges,
        );
        // if this maximal clique is larger then we want to return this at the end
        if cq.len() > best.len() {
            best = cq;
        }
    }
    best
}

#[inline]
fn p2(nodes: &[&str], edges: &HashSet<(usize, usize)>) {
    let max = bron_kerbosch(
        HashSet::new(),
        HashSet::from_iter(0..nodes.len()),
        HashSet::new(),
        edges,
    );
    let mut out = max.iter().map(|&idx| nodes[idx]).collect::<Vec<_>>();
    // sort alphabetically
    out.sort();
    output(out.join(","));
}
