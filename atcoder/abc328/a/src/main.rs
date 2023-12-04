use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
        s: [usize; n]
    }
    let mut ans = 0;
    for i in 0..n {
        if s[i] <= x {
            ans += s[i]
        }
    }
    println!("{}", ans);
}
