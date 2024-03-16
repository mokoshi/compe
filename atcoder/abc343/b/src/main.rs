use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [[u8; n]; n]
    }
    let mut g = vec![vec![]; n];
    for i in 0..n {
        for j in 0..n {
            if a[j][i] == 1 {
                g[j].push(i + 1);
            }
        }
    }
    for i in 0..n {
        println!("{}", g[i].iter().join(" "));
    }
}
