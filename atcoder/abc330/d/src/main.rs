use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: [Chars; n]
    }
    let mut cntc = vec![0; n];
    let mut cntr = vec![0; n];
    for i in 0..n {
        for j in 0..n {
            if s[j][i] == 'o' {
                cntc[i] += 1;
                cntr[j] += 1;
            }
        }
    }

    let mut ans: i64 = 0;
    for i in 0..n {
        for j in 0..n {
            if s[i][j] == 'o' {
                ans += (cntc[j] - 1) * (cntr[i] - 1);
            }
        }
    }
    println!("{}", ans);
}
