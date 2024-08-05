use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        t: i64,
        s: Chars,
        x: [i64; n],
    }
    let mut l = vec![];
    let mut r = vec![];
    for i in 0..n {
        if s[i] == '0' {
            l.push(x[i]);
        } else {
            r.push(x[i]);
        }
    }
    l.sort();
    r.sort();

    let mut cnt = 0;
    for i in 0..r.len() {
        let pl = l.partition_point(|&a| a < r[i]);
        let pr = l.partition_point(|&a| a - r[i] <= 2 * t);
        cnt += pr - pl;
    }
    println!("{}", cnt);
}
