use proconio::input;
use std::collections::VecDeque;

fn main() {
    input! {
        n: usize,
        a: [u32; n]
    }
    let inf = u32::MAX;
    let mut dp = VecDeque::from(vec![inf; n]);
    let mut ans = 0;
    for i in 0..n {
        let a = a[i];
        let p = dp.partition_point(|&x| x < a);
        if p == 0 {
            ans += 1;
            dp.push_front(a);
        } else {
            dp[p - 1] = a;
        }
    }
    println!("{}", ans);
}
