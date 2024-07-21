use proconio::input;

fn main() {
    input! {
        n: usize,
        a: u32,
        t: [u32; n]
    }
    let mut ans = vec![0; n + 1];
    for i in 1..=n {
        ans[i] = ans[i - 1].max(t[i - 1]) + a;
        println!("{}", ans[i]);
    }
}
