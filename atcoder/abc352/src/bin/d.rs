use std::collections::BTreeSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        p: [usize; n]
    }
    let mut q = vec![0; n];
    for i in 0..n {
        q[p[i] - 1] = i + 1;
    }
    let mut s = BTreeSet::new();
    for i in 0..k - 1 {
        s.insert(q[i]);
    }
    let mut ans = n;
    for i in 0..=n - k {
        // i..i+k の範囲で、qの最小値と最大値の差を求める
        s.insert(q[i + k - 1]);
        let d = s.last().unwrap() - s.first().unwrap();
        ans = ans.min(d);
        s.remove(&q[i]);
    }
    println!("{}", ans);
}
