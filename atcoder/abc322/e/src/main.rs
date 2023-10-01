use proconio::input;

const INF: i64 = 1_000_000_000_000;

fn main() {
    input! {
        n: usize,
        k: usize,
        p: usize,
        ca: [(i64, [usize; k]); n]
    }
    let state_num = (p + 1).pow(k as u32);
    let mut dp = vec![vec![INF; state_num]; n + 1];
    dp[0][0] = 0;

    let digit = |i: usize, d: usize| -> usize { (i / (p + 1).pow(d as u32)) % (p + 1) };

    for i in 1..=n {
        let (c, a) = &ca[i - 1];
        for j in 0..state_num {
            let mut prev = j;
            for ki in 0..k {
                let d = digit(j, ki);
                prev -= (if d > a[ki] { a[ki] } else { d }) * (p + 1).pow(ki as u32);
            }
            dp[i][j] = dp[i - 1][j].min(dp[i - 1][prev] + c);
        }
    }

    let ans = dp[n][state_num - 1];
    println!("{}", if ans == INF { -1 } else { ans });
}
