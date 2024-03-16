use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        t: usize,
        ab: [(usize, u64); t]
    }
    let mut m = HashMap::new();
    let mut k = vec![0; n + 1];
    m.insert(0, n);
    for i in 0..t {
        let (a, b) = ab[i];
        let c = *m.get(&k[a]).unwrap();
        if c == 1 {
            m.remove(&k[a]);
        } else {
            m.insert(k[a], c - 1);
        }
        k[a] += b;
        let c = *m.get(&k[a]).unwrap_or(&0);
        m.insert(k[a], c + 1);
        println!("{}", m.len());
    }
}
