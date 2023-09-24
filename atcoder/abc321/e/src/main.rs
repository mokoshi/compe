use std::collections::{HashSet, VecDeque};

use proconio::input;

fn main() {
    input! {
        t: usize
    }
    for _ in 0..t {
        input! {
            n: i64,
            x: i64,
            k: i64
        }

        let mut t = HashSet::new();
        let mut q = VecDeque::new();
        t.insert(x);
        q.push_back((0, x));
        let mut adj = 0;
        while !q.is_empty() {
            let (c, p) = q.pop_front().unwrap();
            if c == k {
                adj = 1;
                break;
            }

            let p1 = p / 2;
            let p2 = p * 2;
            let p3 = p * 2 + 1;
            for pp in [p1, p2, p3] {
                if t.contains(&pp) {
                    continue;
                }
                if pp <= 0 || pp > n {
                    continue;
                }
                t.insert(pp);
                q.push_back((c + 1, pp));
            }
        }
        println!("{}", q.len() + adj);
    }
}
