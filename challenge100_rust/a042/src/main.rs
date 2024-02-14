use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        d: [i64; n],
        c: [i64; m]
    }
    let mut dp = vec![vec![0; n + 1]; m + 1];
    for i in 1..=m {
        for j in 0..=n.min(i) {
            if j == 0 {
                continue;
            }
            let stay = dp[i - 1][j];
            let movec = dp[i - 1][j - 1] + c[i - 1] * d[j - 1];
            dp[i][j] = if i == j { movec } else { movec.min(stay) }
        }
    }
    println!("{}", (n..=m).map(|x| dp[x][n]).min().unwrap());
}
