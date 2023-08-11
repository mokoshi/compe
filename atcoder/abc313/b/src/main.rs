use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(usize, usize); m],
    }

    let mut aset = HashSet::new();
    let mut bset = HashSet::new();
    for (a, b) in ab {
        aset.insert(a);
        bset.insert(b);
    }

    let mut answer = Vec::new();
    for i in 1..=n {
        if aset.contains(&i) && !bset.contains(&i) {
            answer.push(i);
        }
    }

    println!(
        "{}",
        if answer.len() == 1 {
            answer[0] as i32
        } else {
            -1
        }
    );
}
