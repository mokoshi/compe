use proconio::input;
use std::{cmp::Reverse, collections::BinaryHeap};

fn main() {
    input! {
        n: usize,
        abx: [(u64, u64, usize); n-1]
    }
    let mut g = vec![vec![]; n];
    for i in 0..n - 1 {
        let (a, b, x) = abx[i];
        g[i].push((a, i + 1));
        g[i].push((b, x - 1));
    }

    let mut v = vec![0; n];
    let mut q = BinaryHeap::new();
    q.push(Reverse(g[0][0]));
    q.push(Reverse(g[0][1]));
    while !q.is_empty() {
        let (d, np) = q.pop().unwrap().0;
        if np == 0 || v[np] > 0 {
            continue;
        }
        v[np] = d;
        for e in &g[np] {
            q.push(Reverse((e.0 + d, e.1)))
        }
    }
    println!("{}", v[n - 1]);
}
