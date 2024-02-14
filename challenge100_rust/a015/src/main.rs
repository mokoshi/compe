use itertools::Itertools;
use proconio::input;

// https://atcoder.jp/contests/abc145/tasks/abc145_c
fn main() {
    input! {
        n: usize,
        xy: [(i32, i32); n],
    }

    let mut total = 0.0;
    for root in (0..n).permutations(n) {
        for i in 0..root.len() - 1 {
            let (x1, y1) = xy[root[i]];
            let (x2, y2) = xy[root[i + 1]];
            total += (((x1 - x2).pow(2) + (y1 - y2).pow(2)) as f64).sqrt();
        }
    }

    println!("{}", total / (1..=n).product::<usize>() as f64);
}
