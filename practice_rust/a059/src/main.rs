use proconio::input;
use std::{
    cmp::Reverse,
    collections::{BinaryHeap, VecDeque},
};

const INF: usize = usize::MAX / 2;

fn main() {
    input! {
        n: usize,
        k: usize,
        cr: [(usize, usize); n],
        ab: [(usize, usize); k]
    }
    let mut g = vec![vec![]; n];
    for &(a, b) in &ab {
        g[a - 1].push(b - 1);
        g[b - 1].push(a - 1);
    }

    let mut gg = vec![vec![]; n];
    for i in 0..n - 1 {
        let (c, r) = cr[i];
        let mut q = VecDeque::from([i]);
        let mut dist = vec![INF; n];
        dist[i] = 0;
        while let Some(u) = q.pop_front() {
            if u == n - 1 || dist[u] >= r {
                break;
            }
            for &to in &g[u] {
                if dist[to] == INF {
                    dist[to] = dist[u] + 1;
                    q.push_back(to);
                }
            }
        }
        for j in 0..n {
            if dist[j] < INF && i != j {
                gg[i].push(j);
            }
        }
    }

    let mut v = vec![INF; n];
    let mut q = BinaryHeap::from([Reverse((0, 0))]);
    while let Some(Reverse((d, u))) = q.pop() {
        if d >= v[u] {
            continue;
        }
        v[u] = d;
        if u == n - 1 {
            break;
        }
        for &to in &gg[u] {
            let w = cr[u].0;
            if d + w < v[to] {
                q.push(Reverse((d + w, to)));
            }
        }
    }
    println!("{}", v[n - 1]);
}
