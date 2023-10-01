use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [i32; m]
    }
    let mut at = HashSet::new();
    for ai in a {
        at.insert(ai);
    }

    let mut t = vec![0; n];
    let mut cnt = 0;
    for i in 1..=n {
        let j = n as i32 + 1 - i as i32;
        if at.contains(&j) {
            cnt = 0;
        } else {
            cnt += 1;
        }
        t[j as usize - 1] = cnt;
    }
    for ans in t {
        println!("{}", ans);
    }
}
