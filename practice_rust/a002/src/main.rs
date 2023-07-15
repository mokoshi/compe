use proconio::input;

// https://atcoder.jp/contests/abc106/tasks/abc106_b
fn main() {
    input! {
        n: i32,
    }

    let mut answer = 0;
    for i in 1..n + 1 {
        if i % 2 == 0 {
            continue;
        }
        if count_divisors(i) == 8 {
            answer += 1;
        }
    }
    println!("{}", answer)
}

fn count_divisors(n: i32) -> i32 {
    let mut cnt = 0;
    for i in 1..((n as f64).sqrt() as i32) + 1 {
        if n % i == 0 {
            if i * i == n {
                cnt += 1;
            } else {
                cnt += 2;
            }
        }
    }
    cnt
}
