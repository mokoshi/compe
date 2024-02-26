use ac_library::{LazySegtree, MapMonoid, Monoid};
use proconio::input;

struct Additive;
impl Monoid for Additive {
    type S = f64;
    fn identity() -> Self::S {
        0.0
    }
    fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
        *a + *b
    }
}

struct Add;
impl MapMonoid for Add {
    type M = Additive;
    type F = f64;

    fn identity_map() -> Self::F {
        0.0
    }

    fn mapping(&f: &f64, &x: &f64) -> f64 {
        f + x
    }

    fn composition(&f: &f64, &g: &f64) -> f64 {
        f + g
    }
}

fn main() {
    input! {
        n: usize,
        l: usize,
        d: usize,
    }
    let df = d as f64;
    let mut st = LazySegtree::<Add>::new(n);
    st.set(0, 0.0);
    st.set(1, 1.0 / df);
    for i in 1..n {
        let p = st.get(i);
        st.apply_range(i + 1..i + d, p / df);
    }
}
