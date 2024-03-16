use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [u64; n],
        m: usize,
        b: [u64; m],
        l: usize,
        c: [u64; l],
        q: usize,
        x: [u64; q],
    }
    let mut t = HashSet::new();
    for i in 0..n {
        for j in 0..m {
            for k in 0..l {
                t.insert(a[i] + b[j] + c[k]);
            }
        }
    }
    for x in &x {
        if t.contains(&x) {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
