use std::io;

// https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ALDS1_5_A&lang=ja
fn main() {
    let (n, a, _, m) = input();

    'root: for z in &m {
        for i in 0..1 << n {
            let mut sum = 0;
            for j in 0..n {
                if i & 1 << j != 0 {
                    sum += a[j as usize];
                }
            }
            if sum == *z {
                println!("yes");
                continue 'root;
            }
        }
        println!("no");
    }
}

fn input() -> (i32, Vec<i32>, i32, Vec<i32>) {
    let n: i32 = read_line()[0];
    let a = read_line();

    let q: i32 = read_line()[0];
    let m = read_line();
    (n, a, q, m)
}

fn read_line() -> Vec<i32> {
    let mut s = String::new();
    io::stdin().read_line(&mut s).expect("");
    s.trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect()
}
