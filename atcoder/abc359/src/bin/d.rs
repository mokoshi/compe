use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        k: usize,
        s: Chars
    }
    let md: u64 = 998244353;

    let is_pal = (0..1 << k)
        .map(|s| (0..k / 2).all(|i| (s >> i & 1) == (s >> (k - i - 1) & 1)))
        .collect_vec();

    let mut dp = vec![vec![0; 1 << k]; n];
    let mask = (1 << k) - 1;

    for j in 0..1 << k {
        let mut is_ok = true;
        for i in 0..k {
            let c = if (j >> (k - 1 - i)) & 1 == 1 {
                'A'
            } else {
                'B'
            };
            if (s[i] != '?' && s[i] != c) {
                is_ok = false;
                break;
            }
        }
        if is_ok && !is_pal[j] {
            dp[k - 1][j] = 1;
        }
    }
    // printDP(&dp[k - 1], k);

    for i in k - 1..n - 1 {
        for j in 0..1 << k {
            let na = ((j << 1) + 1) & mask;
            let nb = (j << 1) & mask;
            if matches!(s[i + 1], 'A' | '?') && !is_pal[na] {
                dp[i + 1][na] = (dp[i + 1][na] + dp[i][j]) % md;
            }
            if matches!(s[i + 1], 'B' | '?') && !is_pal[nb] {
                dp[i + 1][nb] = (dp[i + 1][nb] + dp[i][j]) % md;
            }
        }
        // printDP(&dp[i + 1], k);
    }

    let mut ans = 0;
    for j in 0..1 << k {
        ans = (ans + dp[n - 1][j]) % md;
    }
    println!("{}", ans);
}

fn printDP(dp: &Vec<u64>, k: usize) {
    for j in 0..1 << k {
        println!("{j:>10b} : {}", dp[j]);
    }
}

// AB?A?BA
