use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i32; n]
    }
    let mut map = HashMap::new();
    for i in 0..n {
        map.insert(a[i], i as i32 + 1);
    }
    let mut s = -1;
    for _ in 0..n {
        s = *map.get(&s).unwrap();
        print!("{} ", s);
    }
    println!()
}
