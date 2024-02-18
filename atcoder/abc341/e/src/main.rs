use ac_library::{Additive, Segtree};
use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        q: usize,
        s: Chars,
        q: [(u8, usize, usize); q]
    }
    let mut a = vec![0; n - 1];
    for i in 0..n - 1 {
        a[i] = if s[i] != s[i + 1] { 1 } else { 0 };
    }

    let mut segtree = Segtree::<Additive<usize>>::from(a);
    for &(op, l, r) in &q {
        if op == 1 {
            if l > 1 {
                segtree.set(l - 2, 1 - segtree.get(l - 2))
            };
            if r < n {
                segtree.set(r - 1, 1 - segtree.get(r - 1))
            }
        } else {
            println!(
                "{}",
                if l == r || segtree.prod(l - 1..r - 1) == r - l {
                    "Yes"
                } else {
                    "No"
                }
            )
        }
    }
}
