use std::io;

// https://judge.u-aizu.ac.jp/onlinejudge/submission.jsp#submit/DPL_1_B
fn main() {
    let (n, w, vv, ww) = input();
    let dp = vec![vec![-1; w + 1]; n];

    struct Env {
        n: usize,
        vv: Vec<i32>,
        ww: Vec<usize>,
        dp: Vec<Vec<i32>>,
    }

    let mut env = Env { n, vv, ww, dp };

    fn knapsack(i: usize, cw: usize, e: &mut Env) -> i32 {
        if i == e.n {
            return 0;
        }
        if e.dp[i][cw] != -1 {
            return e.dp[i][cw];
        }
        let ans = if e.ww[i] > cw {
            knapsack(i + 1, cw, e)
        } else {
            std::cmp::max(
                knapsack(i + 1, cw, e),
                knapsack(i + 1, cw - e.ww[i], e) + e.vv[i],
            )
        };
        e.dp[i][cw] = ans;
        return ans;
    }

    println!("{}", knapsack(0, w, &mut env));
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
