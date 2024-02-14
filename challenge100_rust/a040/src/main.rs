use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        k: usize,
        kk: [(usize, usize); k]
    }
    let kmap: HashMap<_, _> = kk.into_iter().collect();

    let mut dp = vec![vec![vec![0; 3]; 3]; n];
    for i in 0..3 {
        if kmap.contains_key(&1) && kmap.get(&1).unwrap() != &(i + 1) {
            continue;
        }
        for j in 0..3 {
            if kmap.contains_key(&2) && kmap.get(&2).unwrap() != &(j + 1) {
                continue;
            }
            dp[1][i][j] = 1;
        }
    }
    for k in 2..n {
        for i in 0..3 {
            for j in 0..3 {
                if kmap.contains_key(&(k + 1)) && kmap.get(&(k + 1)).unwrap() != &(j + 1) {
                    continue;
                }
                dp[k][i][j] = (dp[k - 1][0][i] + dp[k - 1][1][i] + dp[k - 1][2][i]
                    - if i == j { dp[k - 1][i][i] } else { 0 })
                    % 10000;
            }
        }
    }

    println!(
        "{}",
        dp[n - 1].iter().fold(0, |acc, v| {
            return acc + v.iter().sum::<i64>();
        }) % 10000
    );
}
