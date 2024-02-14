use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        stdtime: [(usize, usize, i64, i64); m],
    }
    let inf = 1_000_000_000_000_000_i64;

    let mut g = vec![vec![(0, 0); n]; n];
    for &(s, t, d, time) in &stdtime {
        g[s - 1][t - 1] = (d, time);
        g[t - 1][s - 1] = (d, time);
    }

    let mut dp = vec![vec![(inf, 0); n]; 1 << n];
    dp[0][0] = (0, 1);
    // for s in 0..1 << n {
    //     for k in 0..n {
    //         if s & (1 << k) != 0 {
    //             continue;
    //         }
    //         let ns = s | (1 << k);
    //         for a in 0..n {
    //             let e = g[a][k];
    //             let ft = dp[s][a].0 + e.0;
    //             if e.0 == 0 || ft > e.1 {
    //                 continue;
    //             }
    //             if ft < dp[ns][k].0 {
    //                 dp[ns][k] = (ft, dp[s][a].1);
    //             } else if ft == dp[ns][k].0 {
    //                 dp[ns][k].1 += dp[s][a].1;
    //             }
    //         }
    //     }
    // }
    for s in 0..1 << n {
        for k in 0..n {
            if s & (1 << k) != 0 {
                continue;
            }
            let ns = s | (1 << k);
            for a in 0..n {
                let e = g[a][k];
                let ft = dp[s][a].0 + e.0;
                if e.0 == 0 || ft > e.1 {
                    continue;
                }
                if ft < dp[ns][k].0 {
                    dp[ns][k] = (ft, dp[s][a].1);
                } else if ft == dp[ns][k].0 {
                    dp[ns][k].1 += dp[s][a].1;
                }
            }
        }
        println!("s: {}", s);
        p(&dp);
        println!("---------");
    }
    // p(&dp);

    let (c, ptn) = dp[(1 << n) - 1][0];
    if c == inf {
        println!("IMPOSSIBLE");
    } else {
        println!("{} {}", c, ptn);
    }
}

/**
3 3
1 2 1 3
2 3 1 3
3 1 1 3

4 5
1 2 1 100
2 4 10 100
2 3 5 100
3 4 5 100
1 4 1 100
 */

fn p(dp: &Vec<Vec<(i64, i32)>>) {
    let n = dp[0].len();
    for s in 0..1 << n {
        for bit in 0..n {
            print!(
                "{}",
                if s & (1 << (n - 1 - bit)) == 0 {
                    "0"
                } else {
                    "1"
                }
            )
        }
        println!("");
        println!("{:?}", dp[s]);
    }
}
