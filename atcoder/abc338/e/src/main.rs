use proconio::input;
use std::{collections::VecDeque, vec};

fn main() {
    input! {
        n: usize,
        ab: [(usize, usize); n],
    }
    let mut g = vec![0; 2 * n];
    for (a, b) in ab {
        g[a - 1] = b - 1;
        g[b - 1] = a - 1;
    }

    let mut q = VecDeque::new();
    for i in 0..2 * n {
        if g[i] > i {
            q.push_back(g[i]);
        } else {
            let p = q.pop_back().unwrap();
            if p != i {
                println!("Yes");
                return;
            }
        }
    }
    println!("No");
}
