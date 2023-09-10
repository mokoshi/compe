use std::io;

// https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ALDS1_10_A&lang=ja
fn main() {
    let n = read_line()[0] as usize;

    let mut fib = vec![0; n + 1];
    fib[0] = 1;
    fib[1] = 1;
    if n <= 1 {
        println!("{}", fib[n]);
        return;
    }
    for i in 2..n + 1 {
        fib[i] = fib[i - 1] + fib[i - 2];
    }
    println!("{}", fib[n]);
}

fn read_line() -> Vec<i32> {
    let mut s = String::new();
    io::stdin().read_line(&mut s).expect("");
    s.trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect()
}
