use proconio::input;

fn main() {
    input! {
        mut n: usize
    }
    let mut n = n - 1;
    let mut ans = 0 as u64;
    for i in 0..=18 {
        let v = 5_usize.pow((18 - i) as u32);
        let k = n / v;
        n -= k * v;
        ans += [0, 2, 4, 6, 8][k] as u64 * 10_u64.pow((18 - i) as u32);
    }
    println!("{}", ans);
}
