use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars
    }

    let mut table = vec![0; 300];
    let mut start = 0;
    let mut ans = 0;
    for i in 0..n {
        let c = s[i];
        if s[start] != c {
            start = i;
        }
        let len = i - start + 1;
        if table[c as usize] < len {
            table[c as usize] = len;
            ans += 1;
        }
    }
    println!("{}", ans);
}
