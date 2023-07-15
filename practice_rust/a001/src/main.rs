use std::io;

// https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ITP1_7_B&lang=ja
fn main() {
    loop {
        let (n, x) = input();
        if (n, x) == (0, 0) {
            break;
        }

        let mut ans = 0;
        for i in 1..(n + 1) {
            for j in (i + 1)..(n + 1) {
                for k in (j + 1)..(n + 1) {
                    if i + j + k == x {
                        ans += 1;
                    }
                }
            }
        }

        println!("{}", ans);
    }
}

fn input() -> (i32, i32) {
    let mut s = String::new();
    io::stdin().read_line(&mut s).expect("");
    let mut iter = s.trim().split_whitespace();

    let n: i32 = iter.next().unwrap().parse().unwrap();
    let x: i32 = iter.next().unwrap().parse().unwrap();

    (n, x)
}
