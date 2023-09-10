use std::io;

// https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=DPL_1_C&lang=ja
fn main() {
    let (n, w, vv, ww) = input();
    let mut dp = vec![vec![0; w + 1]; n + 1];

    for ii in 0..n {
        let i = n - ii - 1;
        for cw in 0..w + 1 {
            dp[i][cw] = if ww[i] > cw {
                dp[i + 1][cw]
            } else {
                *vec![
                    dp[i + 1][cw],
                    dp[i + 1][cw - ww[i]] + vv[i],
                    dp[i][cw - ww[i]] + vv[i],
                ]
                .iter()
                .max()
                .unwrap()
            };
        }
    }

    println!("{}", dp[0][w]);
}

fn input() -> (usize, usize, Vec<i32>, Vec<usize>) {
    let line = read_line();
    let n = line[0] as usize;
    let w = line[1] as usize;

    let mut vv = vec![0; n as usize];
    let mut ww = vec![0; n as usize];
    for i in 0..n {
        let line = read_line();
        vv[i] = line[0];
        ww[i] = line[1] as usize;
    }

    (n, w, vv, ww)
}

fn read_line() -> Vec<i32> {
    let mut s = String::new();
    io::stdin().read_line(&mut s).expect("");
    s.trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect()
}
