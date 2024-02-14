use std::collections::VecDeque;

use proconio::input;

fn main() {
    input! {
        r: usize,
        c: usize,
        s: (usize, usize),
        g: (usize, usize),
        ct: [String; r]
    }
    let mut cc = vec![];
    for i in 0..r {
        cc.push(ct[i].chars().collect::<Vec<char>>());
    }

    let mut dd = vec![vec![-1; c]; r];
    dd[s.0 - 1][s.1 - 1] = 0;

    let mut q = VecDeque::from([(s.0 - 1, s.1 - 1)]);
    while !q.is_empty() {
        let p = q.pop_front().unwrap();
        let d = dd[p.0][p.1];
        for (dx, dy) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
            let ny = (p.0 as i32 + dy) as usize;
            let nx = (p.1 as i32 + dx) as usize;
            if dd[ny][nx] == -1 && cc[ny][nx] == '.' {
                q.push_back((ny, nx));
                dd[ny][nx] = d + 1;
            }
        }
    }
    println!("{}", dd[g.0 - 1][g.1 - 1]);
}
