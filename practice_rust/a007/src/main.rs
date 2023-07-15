use proconio::input;
use std::collections::HashSet;

// https://atcoder.jp/contests/joi2007ho/tasks/joi2007ho_c
fn main() {
    input! {
        n: usize,
        p: [(i32, i32); n]
    };

    let mut map = HashSet::new();
    for i in &p {
        map.insert(i);
    }

    let mut answer = 0;
    for i1 in 0..p.len() {
        for i2 in i1 + 1..p.len() {
            let p1 = p[i1];
            let p2 = p[i2];
            for p3 in &[
                (p2.0 + (p2.1 - p1.1), p2.1 - (p2.0 - p1.0)),
                (p2.0 - (p2.1 - p1.1), p2.1 + (p2.0 - p1.0)),
            ] {
                let p4 = (p1.0 + (p3.0 - p2.0), p1.1 + (p3.1 - p2.1));
                if map.contains(&p3) && map.contains(&p4) {
                    answer = answer.max((p1.0 - p2.0).pow(2) + (p1.1 - p2.1).pow(2));
                }
            }
        }
    }

    println!("{}", answer);
}
