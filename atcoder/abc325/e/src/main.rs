use proconio::input;

fn main() {
    input! {
        n: usize,
        a: i64,
        b: i64,
        c: i64,
        d: [[i64; n]; n]
    }
    let inf = 1_000_000_000;
    let mut dp = vec![vec![inf; n]; n];
    dp[0][0] = 0;
    for i in 0..n {
        dp[1][i] = std::cmp::min(d[i][0] * a, d[i][0] * b + c);
    }
    for i in 1..n {
        for j in i + 1..n {}
    }
}
