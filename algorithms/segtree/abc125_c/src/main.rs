use my_segtree::{Monoid, MySegtree};
use proconio::input;

const INF: u64 = u64::MAX;

struct GcdMonoid {}
impl Monoid for GcdMonoid {
    type S = u64;
    fn identity_element() -> Self::S {
        INF
    }
    fn binary_operation(a: Self::S, b: Self::S) -> Self::S {
        gcd(a, b)
    }
}

fn gcd(a: u64, b: u64) -> u64 {
    if a == INF {
        b
    } else if b == INF {
        a
    } else {
        num::integer::gcd(a, b)
    }
}

fn main() {
    // raw_impl();
    use_my_lib();
}

fn use_my_lib() {
    input! {
        n: usize,
        a: [u64; n],
    }
    let mut segtree = MySegtree::<GcdMonoid>::new(n);
    for i in 0..n {
        segtree.update(i, a[i]);
    }
    let mut ans = 0;
    for i in 0..n {
        let l = segtree.query(0, i);
        let r = segtree.query(i + 1, n);
        ans = ans.max(gcd(l, r));
    }
    println!("{}", ans);
}

fn raw_impl() {
    input! {
        n: usize,
        a: [u64; n],
    }

    // 数列の長さをカバーする最小の2^xを求める
    let ceil_log2 = 32 - (n as u32 - 1).leading_zeros();

    // 葉の数は2^x
    let leave_num = (1 << ceil_log2) as usize;

    let mut segtree = vec![INF; 2 * leave_num - 1];

    // segtree を構築
    for i in 0..n {
        update(i, a[i], leave_num, &mut segtree);
    }

    let mut ans = 0;
    for i in 0..n {
        let l = find(0, i, 0, 0, leave_num, leave_num, &segtree);
        let r = find(i + 1, n, 0, 0, leave_num, leave_num, &segtree);
        ans = ans.max(gcd(l, r));
    }
    println!("{}", ans);
}

fn update(i: usize, x: u64, leave_num: usize, segtree: &mut Vec<u64>) {
    let mut i = leave_num - 1 + i;
    segtree[i] = x;
    while i > 0 {
        // 2分木の親のインデックス: (i-1)/2
        // 2分木の子のインデックス: i*2+1, i*2+2
        i = (i - 1) / 2;
        segtree[i] = gcd(segtree[i * 2 + 1], segtree[i * 2 + 2]);
    }
}
fn find(
    a: usize,
    b: usize,
    k: usize,
    l: usize,
    r: usize,
    leave_num: usize,
    segtree: &Vec<u64>,
) -> u64 {
    // println!("find: {}-{}, k:{}, l-r:{}-{}", a, b, k, l, r);
    if r <= a || l >= b {
        INF
    } else if a <= l && r <= b {
        segtree[k]
    } else {
        let lm = find(a, b, k * 2 + 1, l, (l + r) / 2, leave_num, segtree);
        let rm = find(a, b, k * 2 + 2, (l + r) / 2, r, leave_num, segtree);
        gcd(lm, rm)
    }
}
