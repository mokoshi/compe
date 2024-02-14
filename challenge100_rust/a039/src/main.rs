use proconio::input;

// https://atcoder.jp/contests/joi2011yo/tasks/joi2011yo_d
fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut dp: Vec<Vec<i64>> = vec![vec![0; 21]; n];
    dp[0][a[0]] = 1;

    for i in 1..n - 1 {
        let v = a[i];
        for t in 0..21 {
            let plus = if t >= v { dp[i - 1][t - v] } else { 0 };
            let minus = if t + v <= 20 { dp[i - 1][t + v] } else { 0 };
            dp[i][t] = plus + minus;
        }
    }
    println!("{}", dp[n - 2][a[n - 1]]);
}
