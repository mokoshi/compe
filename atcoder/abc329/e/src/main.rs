use std::collections::VecDeque;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        m: usize,
        mut s: Chars,
        t: Chars
    }
    let mut q = VecDeque::new();
    for i in 0..n - m + 1 {
        if &s[i..i + m] == t {
            q.push_back(i);
        }
    }

    let mut done = 0;
    while !q.is_empty() {
        // println!("start-----------");
        // println!("q: {:?}", q);
        let i = q.pop_front().unwrap();
        for j in i..i + m {
            if s[j] != '#' {
                done += 1;
                s[j] = '#'
            }
        }
        if done == n {
            println!("Yes");
            return;
        }
        // println!("pop: {}, s: {:?}", i, &s);

        let af = (i as i32 - m as i32 + 1).max(0) as usize;
        let ae = (i + m - 1).min(n - m);
        // println!("affected range: ({}..{})", af, ae);
        'root: for j in af..=ae {
            let mut w = 0;
            // println!("for {}, ", j);
            for k in 0..m {
                // println!("check: {}, {}", s[j + k], t[k]);
                if s[j + k] != '#' && s[j + k] != t[k] {
                    continue 'root;
                }
                if s[j + k] == '#' {
                    w += 1;
                }
            }
            if i != j && w != m {
                q.push_back(j);
                // println!("new index: {}", j);
            }
        }
    }
    println!("No");
}
