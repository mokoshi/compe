use proconio::input;

fn main() {
    input! {
        n: usize,
        c: [usize; n]
    }
    let inf = usize::MAX;
    let mut dp = vec![inf; n];
    for i in 0..n {
        let c = c[i];
        let p = dp.partition_point(|&x| x < c);
        dp[p] = c;
    }
    let p = dp.iter().position(|&x| x == inf).unwrap_or(n);
    println!("{}", n - p);
}
