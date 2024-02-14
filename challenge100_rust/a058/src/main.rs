use proconio::input;
use std::{
    cmp::Reverse,
    collections::{BinaryHeap, VecDeque},
    vec,
};

const INF: u64 = u64::MAX / 2;

fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
        s: usize,
        p: u64,
        q: u64,
        c: [usize; k],
        ab: [(usize, usize); m]
    }
    let mut g = vec![vec![]; n];
    for &(a, b) in &ab {
        g[a - 1].push((0, b - 1));
        g[b - 1].push((0, a - 1));
    }

    let mut v = vec![INF; n];
    let mut qu = VecDeque::from(
        c.clone()
            .into_iter()
            .map(|c| (c - 1, 0))
            .collect::<Vec<(usize, u64)>>(),
    );
    while !qu.is_empty() {
        let (p, cost) = qu.pop_front().unwrap();
        if v[p] <= cost {
            continue;
        }
        v[p] = cost;
        if cost >= s as u64 {
            continue;
        }
        for &(_, np) in &g[p] {
            qu.push_back((np, cost + 1));
        }
    }

    // グラフに重みをつける
    for f in 0..n {
        for t in &mut g[f] {
            t.0 = if t.1 == n - 1 {
                0
            } else if v[t.1] == 0 {
                INF
            } else if v[t.1] <= s as u64 {
                q
            } else {
                p
            };
        }
    }

    // 最短経路
    let mut md = vec![INF; n];
    let mut q = BinaryHeap::from([Reverse((0, 0))]);
    while let Some(Reverse((d, u))) = q.pop() {
        if d >= md[u] {
            continue;
        }
        md[u] = d;
        for &(w, j) in &g[u] {
            if d + w < md[j] {
                q.push(Reverse((d + w, j)));
            }
        }
    }
    println!("{}", md[n - 1]);
}
