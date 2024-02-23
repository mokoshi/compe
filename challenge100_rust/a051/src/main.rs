use std::collections::HashMap;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        joi: Chars
    }
    let ci = HashMap::from([('J', 0), ('O', 1), ('I', 2)]);
    let mut dp = vec![vec![0_u32; 8]; n];
    for s in 0..1 << 3 {
        if s & 1 != 0 && s & 1 << ci[&joi[0]] != 0 {
            dp[0][s] = 1;
        }
    }
    for i in 1..n {
        for s in 0..1 << 3 {
            let mut p = 0;
            for t in 0..1 << 3 {
                if s & t != 0 && s & 1 << ci[&joi[i]] != 0 {
                    p += dp[i - 1][t];
                }
            }
            dp[i][s] = p % 10007;
        }
    }
    println!("{}", dp[n - 1].iter().sum::<u32>() % 10007);
}
