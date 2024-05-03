use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars,
        c: [u64; n]
    }
    let inf = u64::MAX / 2;
    let mut dp = vec![vec![vec![inf; 2]; 2]; n];
    dp[0][0][0] = if s[0] == '0' { 0 } else { c[0] };
    dp[0][0][1] = if s[0] == '1' { 0 } else { c[0] };
    for i in 1..n {
        let to0 = if s[i] == '0' { 0 } else { c[i] };
        let to1 = if s[i] == '1' { 0 } else { c[i] };
        dp[i][0][0] = dp[i - 1][0][1] + to0;
        dp[i][0][1] = dp[i - 1][0][0] + to1;
        dp[i][1][0] = (dp[i - 1][1][1] + to0).min(dp[i - 1][0][0] + to0);
        dp[i][1][1] = (dp[i - 1][1][0] + to1).min(dp[i - 1][0][1] + to1);
    }
    println!("{}", dp[n - 1][1][0].min(dp[n - 1][1][1]))
}
