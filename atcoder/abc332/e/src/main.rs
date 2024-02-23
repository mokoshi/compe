use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        d: usize,
        w: [f64; n]
    }
    let ax = w.iter().sum::<f64>() / d as f64;
    let mut dp = vec![vec![0.0; d + 1]; 1 << n];

    for s in 0..1 << n {
        let mut total = 0.0;
        for i in 0..n {
            if s & 1 << i != 0 {
                total += w[i]
            }
        }
        dp[s][1] = (total - ax).powi(2)
    }
    for s in 0..1 << n {
        for k in 2..=d {
            // println!("calc s:{}, k:{}", dg(s), k);
            let mut v = dp[s][k - 1] + dp[0][1];
            let mut t = s;
            while t > 0 {
                // println!(
                //     "v=v.min(dp[{}][{}] + dp[{}][{}])",
                //     dg(s - t),
                //     k - 1,
                //     dg(t),
                //     1
                // );
                v = v.min(dp[s - t][k - 1] + dp[t][1]);
                t = (t - 1) & s;
            }
            dp[s][k] = v;
        }
    }
    // println!("{:?}", dp);
    println!("{}", dp[(1 << n) - 1][d] / d as f64);
}

fn dg(s: usize) -> String {
    let mut g = vec![];
    for i in 0..20 {
        if s & 1 << i != 0 {
            g.push(i);
        }
    }
    format!("({})", g.iter().join(","))
}

/*
3 2
3 4 5

 */
