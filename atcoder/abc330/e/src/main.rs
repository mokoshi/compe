use std::collections::{BTreeSet, BinaryHeap, HashMap};

use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        mut a: [i64; n],
        ix: [(usize, i64); q]
    }
    let mut o = BTreeSet::new();
    let mut t = HashMap::new();
    for &a in &a {
        if t.contains_key(&a) {
            *t.get_mut(&a).unwrap() += 1;
        } else {
            t.insert(a, 1);
            o.insert(a);
        }
    }

    for &(i, x) in &ix {
        let removed = a[i - 1];
        a[i - 1] = x;

        let c = t.get_mut(&removed).unwrap();
        if c == &1 {
            t.remove(&removed);
            o.remove(&removed);
        } else {
            *c -= 1;
        }
        if t.contains_key(&x) {
            *t.get_mut(&x).unwrap() += 1;
        } else {
            t.insert(x, 1);
            o.insert(x);
        }

        println!("{:?}", t);
        println!("{:?}", o);
    }
}

fn find_mex(o: &BTreeSet<i64>) -> i64 {
    let left = 0;
    let right = o.len();
    while left < right {
        let mid = (left + right) / 2;
        o.take(value)
    }
    return 0;
}
