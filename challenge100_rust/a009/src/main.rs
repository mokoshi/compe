use proconio::input;

// https://atcoder.jp/contests/joi2008yo/tasks/joi2008yo_d
fn main() {
    input! {
        m: usize,
        mm: [(i32, i32); m],
        n: usize,
        nn: [(i32, i32); n]
    };

    let mut nset = std::collections::HashSet::new();
    for p in &nn {
        nset.insert(p);
    }

    let s = mm[0];
    for o in &nn {
        let mut ok = true;
        for p in &mm {
            let q = (o.0 + (p.0 - s.0), o.1 + (p.1 - s.1));
            if !nset.contains(&q) {
                ok = false;
                break;
            }
        }
        if ok {
            println!("{} {}", o.0 - s.0, o.1 - s.1);
            return;
        }
    }
}
