use std::io;

// https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ALDS1_10_C&lang=ja
fn main() {
    let (n, m, mut c) = input();
    let mut dp = vec![vec![0; n + 1]; m];

    c.sort();

    for j in 1..=n {
        dp[0][j] = j;
    }
    for i in 1..m {
        for j in 1..=n {
            dp[i][j] = if j < c[i] as usize {
                dp[i - 1][j]
            } else {
                dp[i - 1][j]
                    .min(dp[i - 1][j - c[i] as usize] + 1)
                    .min(dp[i][j - c[i] as usize] + 1)
            };
        }
    }

    let mut ans = std::usize::MAX;
    for i in 0..m {
        ans = ans.min(dp[i][n]);
    }
    println!("{}", ans);
}

fn input() -> (usize, usize, Vec<i32>) {
    let line = read_line();
    let n = line[0] as usize;
    let m = line[1] as usize;
    let c = read_line();

    (n, m, c)
}

fn read_line() -> Vec<i32> {
    let mut s = String::new();
    io::stdin().read_line(&mut s).expect("");
    s.trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect()
}
