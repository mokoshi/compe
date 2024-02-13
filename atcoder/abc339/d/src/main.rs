use std::collections::{HashSet, VecDeque};

use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        b: [Chars; n]
    }
    let mut s = vec![];
    for i in 0..n {
        for j in 0..n {
            if b[i][j] == 'P' {
                s.push((i, j));
            }
        }
    }
    let p1 = s[0];
    let p2 = s[1];

    let mut visited = HashSet::new();
    visited.insert((p1, p2));
    visited.insert((p2, p1));

    let mut q = VecDeque::new();
    q.push_back((p1, p2, 1));
    while !q.is_empty() {
        let (p1, p2, c) = q.pop_front().unwrap();
        // println!("p1: {:?}, p2: {:?}, c: {}", p1, p2, c);
        for (dy, dx) in [(-1, 0), (1, 0), (0, -1), (0, 1)].iter() {
            // println!("dy: {}, dx: {}", dy, dx);
            let mut p1n = ((p1.0 as i32 + dy), (p1.1 as i32 + dx));
            let mut p2n = ((p2.0 as i32 + dy), (p2.1 as i32 + dx));
            if p1n.0 < 0
                || p1n.0 >= n as i32
                || p1n.1 < 0
                || p1n.1 >= n as i32
                || b[p1n.0 as usize][p1n.1 as usize] == '#'
            {
                p1n = (p1.0 as i32, p1.1 as i32);
            }
            if p2n.0 < 0
                || p2n.0 >= n as i32
                || p2n.1 < 0
                || p2n.1 >= n as i32
                || b[p2n.0 as usize][p2n.1 as usize] == '#'
            {
                p2n = (p2.0 as i32, p2.1 as i32);
            }

            let p1nn = (p1n.0 as usize, p1n.1 as usize);
            let p2nn = (p2n.0 as usize, p2n.1 as usize);
            // println!("moved: p1: {:?}, p2: {:?}", p1nn, p2nn);

            if p1nn == p2nn {
                println!("{}", c);
                return;
            }

            if visited.contains(&(p1nn, p2nn)) || visited.contains(&(p2nn, p1nn)) {
                continue;
            }
            visited.insert((p1nn, p2nn));
            visited.insert((p2nn, p1nn));
            q.push_back((p1nn, p2nn, c + 1));
        }
    }
    println!("-1")
}
