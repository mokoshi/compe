use proconio::input;
use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};

fn main() {
    input! {
        n: usize,
        sc: [(i64, i64); n]
    }
    let mut keys = BinaryHeap::new();
    let mut map = HashMap::new();
    for i in 0..n {
        let (s, c) = sc[i];
        map.insert(s, c);
        keys.push(Reverse(s));
    }

    while !keys.is_empty() {
        let s = keys.pop().unwrap().0;
        let c = *map.get(&s).unwrap();
        let a = (c as f64).log2().floor() as i64;
        // println!("{}, {}, {}", s, c, a);
        let mut cc = c;
        while cc > 1 {
            let a = (cc as f64).log2().floor() as u32;
            let ca = 2_i64.pow(a);
            let sa = s * ca;
            // println!("cc,sa,ca: {}, {}, {}", cc, sa, ca);
            cc -= ca;
            *map.get_mut(&s).unwrap() -= ca;
            if map.contains_key(&sa) {
                // println!("{}: {}, {}: {}", s, -ca, sa, 1);
                *map.get_mut(&sa).unwrap() += 1;
            } else {
                keys.push(Reverse(sa));
                map.insert(sa, 1);
            }
        }
    }

    let mut ans = 0;
    for (_, c) in map {
        ans += c;
    }
    println!("{}", ans);
}
