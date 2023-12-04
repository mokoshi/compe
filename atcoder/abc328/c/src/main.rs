use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        q: usize,
        s: Chars,
        lr: [(usize, usize); q]
    }
    let mut t = vec![0; n];
    for i in 1..n {
        if s[i] == s[i - 1] {
            t[i] = t[i - 1] + 1;
        } else {
            t[i] = t[i - 1];
        }
    }
    for i in 0..q {
        let (l, r) = lr[i];
        println!("{}", t[r - 1] - t[l - 1]);
    }
}
