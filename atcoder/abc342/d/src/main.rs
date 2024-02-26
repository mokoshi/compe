use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n]
    }
    let mut ans = 0;
    let mut zeros = 0;
    let mut map = HashMap::new();
    for i in 0..n {
        if a[i] == 0 {
            zeros += 1;
            continue;
        }
        let f = fa(a[i]);
        if !map.contains_key(&f) {
            map.insert(f, 1);
        } else {
            let c = map.get(&f).unwrap();
            map.insert(f, c + 1);
        }
    }

    ans += ((n - 1) + (n - zeros)) * zeros / 2;
    for &cnt in map.values() {
        ans += cnt * (cnt - 1) / 2;
    }
    println!("{}", ans);
}

fn fa(a: usize) -> Vec<usize> {
    let mut a = a;
    let sq = (a as f64).sqrt() as usize;
    let mut ret = vec![];
    for i in 2..=sq {
        let mut cnt = 0;
        while a % i == 0 {
            a /= i;
            cnt += 1;
        }
        if cnt % 2 == 1 {
            ret.push(i);
        }
    }
    if a != 1 {
        ret.push(a)
    }
    ret
}
