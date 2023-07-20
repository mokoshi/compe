use std::collections::HashSet;

use proconio::input;

// https://atcoder.jp/contests/abc002/tasks/abc002_4
fn main() {
    input! {
        n: usize,
        m: usize,
        xy: [(usize, usize); m],
    }

    let mut xyset = HashSet::new();
    for (x, y) in xy {
        xyset.insert((x, y));
    }

    let mut answer = 0;
    for i in 0..1 << n {
        let mut ok = true;
        for j in 1..n + 1 {
            for k in j + 1..n + 1 {
                if i & 1 << (j - 1) != 0 && i & 1 << (k - 1) != 0 {
                    if xyset.contains(&(j, k)) {
                        continue;
                    } else {
                        ok = false;
                        break;
                    }
                }
            }
        }

        if ok {
            answer = answer.max((i as i32).count_ones())
        }
    }

    println!("{}", answer);
}
