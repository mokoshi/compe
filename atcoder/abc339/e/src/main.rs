use ac_library::{Max, Segtree};
use proconio::input;

fn main() {
    input! {
        n: usize,
        d: usize,
        a: [i64; n]
    }
    let mut tree: Segtree<Max<i64>> = Segtree::new(500010);
    tree.set(a[0] as usize, 1);
    for i in 1..n {
        let l = (a[i] - d as i64).max(0) as usize;
        let r = (a[i] + d as i64).min(500010 - 1) as usize;
        let max = tree.prod(l..r + 1).max(0);
        tree.set(a[i] as usize, max + 1);
    }
    println!("{}", tree.all_prod());
}
