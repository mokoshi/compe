use std::{cmp::Reverse, collections::BinaryHeap};

use proconio::input;

fn main() {
    input! {
        n: usize,
        mut td: [(i64, i64); n]
    }

    td.sort();
    td.push((std::i64::MAX, 1));

    let mut ans = 0;
    let mut q = BinaryHeap::new();
    let mut i = 0;
    while i < n {
        let a = td[i];
        // println!("time: {}, item in: {:?}", a.0, a);
        q.push(Reverse(a.0 + a.1));
        while i < n {
            i += 1;
            if a.0 == td[i].0 {
                // println!("time: {}, item in: {:?}", td[i].0, td[i]);
                q.push(Reverse(td[i].0 + td[i].1));
            } else {
                break;
            }
        }
        // println!("in phase end, queue: {:?}", q);

        let mut t = a.0;
        while !q.is_empty() {
            let b = q.pop().unwrap().0;
            if b >= t {
                // println!("time: {}, printed: ~{}. queue: {:?}", t, b, q);
                ans += 1;
                t += 1;
            }
            if t >= td[i].0 {
                // println!("time: {}, next item will in, break", t);
                break;
            }
        }
    }
    println!("{}", ans);
}
