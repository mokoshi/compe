use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; n]
    }
    let mut ac = vec![vec![0; m]; n];
    for i in 0..n {
        for j in 0..m {
            ac[i][j] = if i > 0 { ac[i - 1][j] } else { 0 };
            if a[i] - 1 == j {
                ac[i][j] += 1;
            }
        }
    }

    let cnt = |k: usize, s: usize| -> usize {
        let t = ac[n - 1][k];
        t - (ac[s + t - 1][k] - if s > 0 { ac[s - 1][k] } else { 0 })
    };

    let mut fixed = vec![0; 1 << m];
    for i in 0..1 << m {
        for k in 0..m {
            if i & 1 << k != 0 {
                fixed[i] += ac[n - 1][k];
            }
        }
    }

    let mut dp = vec![usize::MAX; 1 << m];
    for k in 0..m {
        dp[1 << k] = cnt(k, 0);
    }
    for i in 0..1 << m {
        for k in 0..m {
            if i & 1 << k == 0 {
                continue;
            }
            let ip = i ^ 1 << k;
            if ip == 0 {
                continue;
            }
            dp[i] = dp[i].min(dp[ip] + cnt(k, fixed[ip]));
        }
    }
    println!("{}", dp[(1 << m) - 1])
}
