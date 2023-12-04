use ac_library::inv_mod;
use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n]
    }
    let m = 998244353;
    let invm = inv_mod(n as i64, m);
    let mut e = vec![0_i64; n + 1];
    let mut sum = 0;
    e[0] = 0_i64;
    for i in 1..=n {
        sum = (sum + a[n - i] + e[i - 1]) % m;
        e[i] = (sum * invm) % m;
    }
    println!("{}", e[n]);
}
