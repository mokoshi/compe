use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
        t: Chars
    }
    for w in 1..s.len() {
        for i in 0..w {
            if i + t.len() * w < s.len() {
                continue;
            }
            let mut ok = true;
            for j in 0..t.len() {
                let c = i + j * w;
                if c >= s.len() || s[c] != t[j] {
                    ok = false;
                    break;
                }
            }
            if ok {
                println!("Yes");
                return;
            }
        }
    }
    println!("No");
}
