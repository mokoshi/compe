use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        b: [u64; n]
    }
    let mut a = b.clone();
    a.sort();
    a.reverse();

    let mut t = HashMap::new();
    let mut acc = a[0];
    t.insert(a[0], 0);
    for i in 1..n {
        if a[i] != a[i - 1] {
            t.insert(a[i], acc);
        };
        acc += a[i];
    }

    for i in 0..n {
        print!("{} ", t.get(&b[i]).unwrap())
    }
    println!()
}
