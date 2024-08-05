use std::{collections::BinaryHeap, vec};

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
        w: [u64; n],
    }
    let mut q = BinaryHeap::new();
    for i in 0..n {
        q.push((w[i], a[i]));
    }
    let mut ans = 0;
    let mut b = vec![false; n];
    while !q.is_empty() {
        let (w, a) = q.pop().unwrap();
        if !b[a - 1] {
            b[a - 1] = true;
        } else {
            ans += w;
        }
    }
    println!("{}", ans);
}
