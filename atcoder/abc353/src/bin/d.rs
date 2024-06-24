use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [u64; n]
    }
    let mut ds = vec![0; n];
    let mut ss = vec![0; n];
    for i in 1..n {
        ds[i] = ds[i - 1] + 10_u64.pow((a[i] as f64).log10() as u32 + 1);
        ss[i] = ss[i - 1] + a[i];
    }
    // println!("{:?}", ds);
    // println!("{:?}", ss);
    let div = 998244353;
    let mut ans = 0;
    for i in 0..n - 1 {
        ans += a[i] * ((ds[n - 1] - ds[i]) % div) + (ss[n - 1] - ss[i]);
        ans %= div;
    }
    println!("{}", ans);
}
