use std::{cmp::Reverse, collections::BinaryHeap, vec};

fn main() {
    let l = read_line();
    let vn = l[0] as usize;
    let en = l[1] as usize;
    let r = l[2] as usize;

    let mut g = vec![vec![]; vn];
    for _ in 0..en {
        let l = read_line();
        g[l[0] as usize].push((l[2], l[1] as usize));
    }

    let mut v = vec![std::i32::MAX; vn];
    let mut q = BinaryHeap::new();
    q.push(Reverse((0, r)));
    while let Some(Reverse((d, u))) = q.pop() {
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
    for i in 0..vn {
        if v[i] == std::i32::MAX {
            println!("INF");
        } else {
            println!("{}", v[i]);
        }
    }
}

fn read_line() -> Vec<i32> {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).expect("");
    s.trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect()
}
