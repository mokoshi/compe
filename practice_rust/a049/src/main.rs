fn main() {
    let inf = 1_000_000_000;

    let a = read_line();
    let v = a[0] as usize;
    let e = a[1] as usize;
    let mut g = vec![vec![inf; v]; v];
    for _ in 0..e {
        let a = read_line();
        let s = a[0] as usize;
        let t = a[1] as usize;
        let d = a[2];
        g[s][t] = d;
    }

    let mut dp = vec![vec![inf; v]; 1 << v];
    dp[0][0] = 0;
    // dp[S ∪ j][j] = min(dp[S][u] + cost(u, v))
    for s in 0..1 << v {
        for j in 0..v {
            // j を含む次の集合 nextS を計算
            let ns = s | (1 << j);

            // 集合 nextS == S の場合、集合 S がそもそも j を含んでいることになるのでスルー
            if s == ns {
                continue;
            }

            for k in 0..v {
                dp[ns][j] = dp[ns][j].min(dp[s][k] + g[k][j])
            }
        }
    }
    println!(
        "{}",
        if dp[(1 << v) - 1][0] == inf {
            -1
        } else {
            dp[(1 << v) - 1][0]
        }
    )
}

fn read_line() -> Vec<i32> {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).expect("");
    s.trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect()
}
