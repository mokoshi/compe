use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let mut c = vec![0; n];
    let mut aa: Vec<HashSet<i32>> = vec![HashSet::new(); n];
    for i in 0..n {
        input! {
            ci: usize,
            ac: [i32; ci],
        }
        c[i] = ci;
        for acc in ac {
            aa[i].insert(acc);
        }
    }

    input! {
        x: i32
    }

    let mut p = vec![];
    let mut min = 100;
    for i in 0..n {
        if aa[i].contains(&x) {
            if min > c[i] {
                min = c[i];
                p = vec![(i + 1).to_string()];
            } else if min == c[i] {
                p.push((i + 1).to_string());
            }
        }
    }

    println!("{}", p.len());
    println!("{}", p.join(" "));
}
