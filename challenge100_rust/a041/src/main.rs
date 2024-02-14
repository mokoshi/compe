use proconio::input;

fn main() {
    input! {
        d: usize,
        n: usize,
        t: [i32; d],
        abc: [(i32, i32, i32); n]
    }
    let mut dp = vec![vec![0; n]; d];

    for i in 0..n {
        let (a, b, _) = abc[i];
        dp[0][i] = if a <= t[0] && t[0] <= b { 0 } else { -1 };
    }
    for i in 1..d {
        for j in 0..n {
            let (a, b, c) = abc[j];
            dp[i][j] = if a <= t[i] && t[i] <= b {
                (0..n)
                    .filter(|k| dp[i - 1][*k] != -1)
                    .map(|k| dp[i - 1][k] + (c - abc[k].2).abs())
                    .max()
                    .unwrap()
            } else {
                -1
            };
        }
    }
    println!("{}", dp[d - 1].iter().max().unwrap());
}
