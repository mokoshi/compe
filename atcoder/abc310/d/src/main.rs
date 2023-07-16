use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        t: usize,
        m: usize,
        ab: [(usize, usize); m]
    }

    let mut abhash = HashSet::new();
    for p in ab {
        abhash.insert(p);
    }

    println!("{}", rec(&vec![vec![1]], 2, &abhash, n, t));
}

fn rec(
    groups: &Vec<Vec<usize>>,
    b: usize,
    ab: &HashSet<(usize, usize)>,
    n: usize,
    t: usize,
) -> i32 {
    let mut sum = 0;

    if b == n + 1 && groups.len() == t {
        return 1;
    }
    if b > n || groups.len() > t {
        return 0;
    }

    'root: for i in 0..groups.len() {
        for j in 0..groups[i].len() {
            if ab.contains(&(groups[i][j], b)) {
                continue 'root;
            }
        }
        sum += rec(
            &groups
                .iter()
                .map(|g| {
                    if &groups[i] == g {
                        let mut new_g = g.to_vec();
                        new_g.push(b);
                        new_g
                    } else {
                        g.to_vec()
                    }
                })
                .collect(),
            b + 1,
            ab,
            n,
            t,
        );
    }

    let mut new_g = groups.clone();
    new_g.push(vec![b]);

    return sum + rec(&new_g, b + 1, ab, n, t);
}
