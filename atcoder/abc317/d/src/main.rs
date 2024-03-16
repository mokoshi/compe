use proconio::input;

fn main() {
    input! {
        n: usize,
        xyz: [(u64, u64, usize); n]
    }
    let mut c = vec![(0, 0); n];
    let mut tz = 0;
    for i in 0..n {
        let (x, y, z) = xyz[i];
        if x > y {
            c[i] = (0, z)
        } else {
            c[i] = ((y - x) / 2 + 1, z);
        }
        tz += z;
    }

    let inf = u64::MAX / 2;
    let mut dp = vec![vec![inf; 100001]; n];
    for i in 0..=c[0].1 {
        dp[0][i] = c[0].0;
    }
    for i in 1..n {
        dp[i][0] = 0;
        for j in 1..=100000 {
            dp[i][j] = dp[i - 1][j];
            if j > c[i].1 {
                dp[i][j] = dp[i][j].min(dp[i - 1][j - c[i].1] + c[i].0);
            } else {
                dp[i][j] = dp[i][j].min(c[i].0);
            }
        }
    }
    for i in (tz / 2) + 1..=100000 {
        if dp[n - 1][i] < inf {
            println!("{}", dp[n - 1][i]);
            break;
        }
    }
}
