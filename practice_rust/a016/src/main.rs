use itertools::Itertools;
use proconio::input;

// https://atcoder.jp/contests/abc150/tasks/abc150_c
fn main() {
    input! {
        n: i32,
        p: [i32; n],
        q: [i32; n],
    }

    if p == q {
        println!("0");
        return;
    }

    let mut first = 0;
    for (i, v) in (1..=n).permutations(n as usize).enumerate() {
        if v == p || v == q {
            if first != 0 {
                println!("{}", i + 1 - first);
                return;
            } else {
                first = i + 1;
            }
        }
    }
}
