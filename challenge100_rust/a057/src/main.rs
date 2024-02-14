use std::{cmp::Reverse, collections::BinaryHeap};
use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
    }
    let mut g = vec![vec![]; n + 1];
    for _ in 0..k {
        input! { t: u8 };
        if t == 0 {
            input! { a: usize, b: usize };
            println!("{}", dijkstra(&g, a, b));
        } else {
            input! { c: usize, d: usize, e: i32 };
            g[c].push((e, d));
            g[d].push((e, c));
        }
    }
}

fn dijkstra(g: &Vec<Vec<(i32, usize)>>, s: usize, t: usize) -> i32 {
    let mut v = vec![std::i32::MAX; g.len()];
    let mut q = BinaryHeap::new();
    q.push(Reverse((0, s)));
    while let Some(Reverse((d, u))) = q.pop() {
        if u == t {
            return d;
        }
        if d >= v[u] {
            continue;
        }
        v[u] = d;
        for &(w, to) in &g[u] {
            if d + w < v[to] {
                q.push(Reverse((d + w, to)));
            }
        }
    }
    -1
}
