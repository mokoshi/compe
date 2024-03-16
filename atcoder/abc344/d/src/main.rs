use proconio::{input, marker::Chars};

fn main() {
    input! {
        t: Chars,
        n: usize,
    }
    let mut a = vec![];
    let mut s = vec![];
    for _ in 0..n {
        input! { aa: usize, ss: [String; aa] }
        a.push(aa);
        s.push(ss);
    }
    let inf = u32::MAX / 2;
    let mut dp = vec![vec![inf; t.len() + 1]; n];
    for i in 0..n {
        dp[i][0] = 0;
    }
    for i in 0..n {
        for k in 1..=t.len() {
            let mut m = if i == 0 { inf } else { dp[i - 1][k] };
            for s in &s[i] {
                if k < s.len() {
                    continue;
                }
                let mut ok = true;
                for (o, c) in s.chars().enumerate() {
                    if t[k - s.len() + o] != c {
                        ok = false;
                        break;
                    }
                }
                if ok {
                    m = m.min(if i == 0 {
                        if k == s.len() {
                            1
                        } else {
                            inf
                        }
                    } else {
                        dp[i - 1][k - s.len()] + 1
                    });
                }
            }
            dp[i][k] = m;
        }
    }
    if dp[n - 1][t.len()] < inf {
        println!("{}", dp[n - 1][t.len()]);
    } else {
        println!("-1");
    }
}

/*
abcde
3
3 b c abcd
5 abc f abc cd bcde
2 e de

abcde
5
3 a c bcd
5 abc b abc cd cde
2 e c
3 b d bcd
5 abc e abc cd cde

*/
