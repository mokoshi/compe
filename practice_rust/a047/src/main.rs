use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n]
    }
    let ti = |x: usize, d: i32| ((x as i32 + n as i32 + d) % n as i32) as usize;
    let mut dp = vec![vec![0; n]; n];
    for i in 1..=(n + 1) / 2 {
        let w = (i * 2 - (n + 1) % 2) as i32;
        for s in 0..n {
            if w == 1 {
                dp[s][ti(s, 1)] = 0;
                continue;
            }
            let e = ti(s, w);
            dp[s][e] = if a[s] < a[ti(e, -1)] {
                std::cmp::max(
                    a[ti(e, -2)] + dp[s][ti(e, -2)],
                    a[s] + dp[ti(s, 1)][ti(e, -1)],
                )
            } else {
                std::cmp::max(
                    a[ti(s, 1)] + dp[ti(s, 2)][e],
                    a[ti(e, -1)] + dp[ti(s, 1)][ti(e, -1)],
                )
            }
        }
    }

    let mut ans = 0;
    for i in 0..n {
        ans = ans.max(a[i] + dp[ti(i, 1)][i]);
    }
    println!("{}", ans);
}
