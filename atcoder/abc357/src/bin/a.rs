use proconio::input;

fn main() {
    input! {
        n: usize,
        mut m: usize,
        h: [usize; n]
    }
    let mut ans = 0;
    for i in 0..n {
        if m >= h[i] {
            m -= h[i];
            ans += 1;
        } else {
            break;
        }
    }
    println!("{}", ans);
}
