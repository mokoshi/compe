use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        mut s: Chars,
        q: usize,
        cd: [(char, char); q]
    }
    let mut conv = vec!['a'; 26];
    for c in 'a'..='z' {
        conv[c as usize - 'a' as usize] = c;
    }
    for &(c, d) in &cd {
        for i in 0..26 {
            if conv[i] == c {
                conv[i] = d;
            }
        }
    }
    for i in 0..n {
        s[i] = conv[s[i] as usize - 'a' as usize];
    }
    println!("{}", s.iter().collect::<String>());
}
