use proconio::input;
use std::collections::BinaryHeap;

fn main() {
    input! {
        n: usize,
        m: usize,
        ldkcab: [(u64, u64, u64, u64, usize, usize); m]
    }
    let mut g = vec![vec![]; n];
    for &(l, d, k, c, a, b) in &ldkcab {
        g[b - 1].push((c, a - 1, l, d, k))
    }
    let inf = u64::MAX;
    let mut v = vec![inf; n];
    let mut q = BinaryHeap::new();
    q.push((inf, n - 1));
    while !q.is_empty() {
        let (mc, p) = q.pop().unwrap();
        if v[p] != inf {
            continue;
        }
        v[p] = mc;
        for &(c, t, l, d, k) in &g[p] {
            if mc < l + c {
                continue;
            }
            let kk = ((mc - l - c) / d).min(k - 1);
            let m = l + kk * d;
            // println!("t: {}, kk: {}, m: {}", t, kk, m);
            q.push((m, t));
        }
    }
    for i in 0..n - 1 {
        if v[i] == inf {
            println!("Unreachable");
        } else {
            println!("{}", v[i]);
        }
    }
}
