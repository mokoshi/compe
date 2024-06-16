use im_rc::HashSet;
use petgraph::unionfind::UnionFind;
use proconio::input;
use std::{cmp::Reverse, collections::BinaryHeap};

fn main() {
    input! {
        n: usize,
        m: usize,
    }
    let mut cka = BinaryHeap::new();
    let mut ss = HashSet::new();
    for _ in 0..m {
        input! {
            k: usize,
            c: u64,
            a: [usize; k]
        }
        for i in 0..k {
            ss.insert(a[i]);
        }
        cka.push(Reverse((c, k, a)));
    }
    if ss.len() != n {
        println!("-1");
        return;
    }

    let mut uf = UnionFind::<usize>::new(n);
    let mut w = 0;
    let mut ec = 0;
    'root: while !cka.is_empty() {
        let (c, k, a) = cka.pop().unwrap().0;
        for u in 0..k - 1 {
            for v in u + 1..k {
                let (u, v) = (a[u] - 1, a[v] - 1);
                if uf.find(u) == uf.find(v) {
                    continue;
                }
                uf.union(u, v);
                w += c;
                ec += 1;
                if ec == n - 1 {
                    break 'root;
                }
            }
        }
    }
    if ec == n - 1 {
        println!("{}", w);
    } else {
        println!("-1");
    }
}
