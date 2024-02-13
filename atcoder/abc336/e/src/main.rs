use proconio::input;

fn main() {
    input! {
        n: i64
    }
    let mut dp = vec![vec![vec![vec![0_i64; 2]; 9 * 14 + 1]; 9 * 14 + 1]; 14];
    dp[0][0][0][1] = 1;
    for i in 1..=9 * 14 {
        for j in 0..=9 * i {
            dp[i]
        }
    }
}
