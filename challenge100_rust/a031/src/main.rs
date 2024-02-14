use proconio::input;
use std::collections::{HashMap, VecDeque};

type Line = ((i32, i32), (i32, i32));

fn main() {
    input! {
        w: usize,
        h: usize,
        b: [[usize; w]; h],
    }
    let mut bb = vec![vec![0; w + 2]; h + 2];
    for i in 0..w {
        for j in 0..h {
            bb[j + 1][i + 1] = b[j][i];
        }
    }

    let mut result = HashMap::<Line, i32>::new();
    let mut q = VecDeque::<Line>::new();

    let fl = get_line((0, 0), (0, 1));
    q.push_back(fl);
    result.insert(fl, 0);

    let in_bound = |p: &(i32, i32)| -> bool {
        return p.0 >= 0 && p.0 <= w as i32 + 1 && p.1 >= 0 && p.1 <= h as i32 + 1;
    };

    while !q.is_empty() {
        let (p1, p2) = q.pop_front().unwrap();
        let (np1, np2) = if p1.1 == p2.1 && p1.1 % 2 == 0 {
            ((p1.0, p1.1 - 1), (p1.0, p1.1 + 1))
        } else if p1.1 == p2.1 && p1.1 % 2 == 1 {
            ((p2.0, p2.1 - 1), (p2.0, p2.1 + 1))
        } else if (p1.1 % 2 == 0 && p1.0 <= p2.0) || (p1.1 % 2 == 1 && p1.0 < p2.0) {
            ((p1.0 + 1, p1.1), (p2.0 - 1, p2.1))
        } else {
            ((p1.0 - 1, p1.1), (p2.0 + 1, p2.1))
        };
        for ps in [p1, p2] {
            for &pe in [np1, np2].iter().filter(|&p| in_bound(p)) {
                let nl = get_line(ps, pe);
                if result.contains_key(&nl) {
                    continue;
                }
                let b1 = bb[ps.1 as usize][ps.0 as usize];
                let b2 = bb[pe.1 as usize][pe.0 as usize];
                if b1 == 1 && b2 == 1 {
                    continue;
                }

                q.push_back(nl);
                result.insert(nl, if b1 == b2 { 0 } else { 1 });
            }
        }
    }
    println!("{:?}", result.values().sum::<i32>());
}

fn get_line(p1: (i32, i32), p2: (i32, i32)) -> Line {
    return if p1.0 < p2.0 {
        (p1, p2)
    } else if p1.0 > p2.0 {
        (p2, p1)
    } else if p1.1 < p2.1 {
        (p1, p2)
    } else {
        (p2, p1)
    };
}
