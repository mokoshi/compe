use std::collections::{HashMap, HashSet};

fn main() {
    let n = read_line()[0];
    let mut vv = HashMap::new();
    for _ in 0..n {
        let a = read_line();
        let u = a[0];
        let k = a[1] as usize;
        let mut v = HashSet::new();
        for j in 0..k {
            v.insert(a[j + 2]);
        }
        vv.insert(u, v);
    }

    let mut q = vec![1];
    let mut vd = HashMap::new();
    vd.insert(1, 0);
    loop {
        let v = q.remove(0);
        let nvset = vv.get(&v).unwrap();
        for &nv in nvset.iter() {
            if !vd.contains_key(&nv) {
                vd.insert(nv, vd.get(&v).unwrap() + 1);
                q.push(nv);
            }
        }
        if q.len() == 0 {
            break;
        }
    }

    for v in 1..=n {
        println!("{} {}", v, vd.get(&v).unwrap_or(&-1));
    }
}

fn read_line() -> Vec<i32> {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).expect("");
    s.trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect()
}
