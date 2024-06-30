use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: [Chars; n]
    }
    let mut ans = 0;
    for i in 0..n {
        if s[i][0] == 'T' {
            ans += 1;
        }
    }
    println!("{}", ans);
}
