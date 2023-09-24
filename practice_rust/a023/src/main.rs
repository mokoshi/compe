use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        m: i64,
        p: [i64; n]
    }

    let mut t = HashSet::new();
    for i in 0..=n {
        for j in 0..=n {
            t.insert(if i == n { 0 } else { p[i] } + if j == n { 0 } else { p[j] });
        }
    }

    let mut ts = t.iter().collect::<Vec<&i64>>();
    ts.sort();

    let mut ans = 0;
    for &i in &ts {
        let remain = m - i;
        if remain < 0 {
            continue;
        }
        let pos = ts.partition_point(|&x| x <= &remain);
        if pos == 0 {
            ans = ans.max(*i);
        } else {
            ans = ans.max(i + ts[pos - 1]);
        }
    }
    println!("{}", ans);
}
