use ac_library::{Additive, LazySegtree, MapMonoid};
use proconio::input;

fn main() {
    input! {
        n: i64,
        m: usize,
        x: [usize; m],
    }
    // println!("{}", imos(n, m, x));
    println!("{}", segtree(n, m, x));
}

struct Add;
impl MapMonoid for Add {
    type M = Additive<u64>;
    type F = (u64, u64);

    fn identity_map() -> Self::F {
        (0, 0)
    }
    fn mapping(
        f: &Self::F,
        x: &<Self::M as ac_library::Monoid>::S,
    ) -> <Self::M as ac_library::Monoid>::S {
        (f.0 + x).max(f.1)
    }
    fn composition(f: &Self::F, g: &Self::F) -> Self::F {
        (f.0 + g.0, f.1.max(g.1))
    }
}

fn segtree(n: i64, m: usize, x: Vec<usize>) -> i64 {
    let n = 10;
    let mut segtree: LazySegtree<Add> = LazySegtree::new(n as usize);
    let a = segtree.prod(0..n as usize - 1);
    segtree.apply_range(0..1, (3, 0));
    println!("{:?}", segtree);
    segtree.apply_range(0..2, (4, 0));
    println!("{:?}", segtree);
    segtree.apply_range(4..7, (9, 0));
    println!("{:?}", segtree);
    println!("{:?}", segtree.all_prod());
    // for i in 0..m - 1 {
    //     let s = x[i].min(x[i + 1]) - 1;
    //     let e = x[i].max(x[i + 1]) - 1;
    //     let d = (e - s) as i64;

    //     let s2e = segtree.prod(s..e + 1);
    // }
    return 0;
}

fn imos(n: i64, m: usize, x: Vec<usize>) -> i64 {
    let mut dv = vec![0_i64; n as usize];
    for i in 0..m - 1 {
        let s = x[i].min(x[i + 1]) - 1;
        let e = x[i].max(x[i + 1]) - 1;
        let d = (e - s) as i64;
        dv[s] += n - d - d;
        dv[e] += d - (n - d);
        dv[0] += d;
        // println!("s: {}, e: {}, d: {}", s, e, d);
        // println!("{:?}", dv);
    }

    let mut c = vec![0_i64; n as usize];
    c[0] = dv[0];
    for i in 1..n as usize {
        c[i] = c[i - 1] + dv[i];
    }
    return *c.iter().min().unwrap();
}
