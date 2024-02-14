use std::collections::{HashSet, VecDeque};

fn main() {
    loop {
        let a = read_line();
        let w = a[0] as usize;
        let h = a[1] as usize;
        if w == 0 && h == 0 {
            break;
        }
        let mut bv = vec![HashSet::new(); h];
        let mut bh = vec![HashSet::new(); h - 1];
        for i in 0..h * 2 - 1 {
            let a = read_line();
            if i % 2 == 0 {
                for (index, &v) in a.iter().enumerate() {
                    if v == 1 {
                        bv[i / 2].insert(index as i32);
                    }
                }
            } else {
                for (index, &v) in a.iter().enumerate() {
                    if v == 1 {
                        bh[i / 2].insert(index as i32);
                    }
                }
            }
        }

        let mut table = vec![vec![-1; h]; w];
        table[0][0] = 0;
        let mut q = VecDeque::new();
        q.push_back((0, 0));
        while !q.is_empty() {
            let p = q.pop_front().unwrap();
            for &(dx, dy) in [(0, 1), (0, -1), (1, 0), (-1, 0)].iter() {
                if (p.0 == 0 && dx == -1)
                    || (p.0 == w as i32 - 1 && dx == 1)
                    || (p.1 == 0 && dy == -1)
                    || (p.1 == h as i32 - 1 && dy == 1)
                {
                    continue;
                }
                if (dx == 1 && bv[p.1 as usize].contains(&p.0))
                    || (dx == -1 && bv[p.1 as usize].contains(&(p.0 - 1)))
                    || (dy == 1 && bh[p.1 as usize].contains(&p.0))
                    || (dy == -1 && bh[(p.1 - 1) as usize].contains(&p.0))
                {
                    continue;
                }
                let np = (p.0 + dx, p.1 + dy);
                if table[np.0 as usize][np.1 as usize] == -1 {
                    table[np.0 as usize][np.1 as usize] = table[p.0 as usize][p.1 as usize] + 1;
                    q.push_back(np);
                }
            }
        }
        println!("{}", table[w - 1][h - 1] + 1);
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
