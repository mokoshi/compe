use proconio::input;
use std::{
    cmp::Reverse,
    collections::{BTreeSet, BinaryHeap},
};

fn main() {
    input! {
        n: usize,
        m: usize,
        tws: [(i64, i64, i64); m]
    }

    let mut timeline = BinaryHeap::new();
    for (t, w, s) in tws {
        timeline.push(Reverse((t, 1_u8, w, s)));
    }
    let mut queue = BTreeSet::new();
    for i in 0..n {
        queue.insert(i);
    }

    let mut result = vec![0; n];
    while !timeline.is_empty() {
        let (t, e, w_or_p, s) = timeline.pop().unwrap().0;
        if e == 0 {
            // 列に戻る
            queue.insert(w_or_p as usize);
        } else {
            // そうめん流し
            match queue.pop_first() {
                Some(p) => {
                    result[p] += w_or_p;
                    timeline.push(Reverse((t + s, 0, p as i64, 0)))
                }
                None => {}
            }
        }
    }
    for i in 0..n {
        println!("{}", result[i]);
    }
}
