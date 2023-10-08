use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
        t: [i32; n]
    }
    let mut dp = vec![vec![0; x]; x];
    for i in 0..x {
        for j in 0..i {
            for k in 0..n {
                dp[i][j] = 
            }
        }
    }
}
