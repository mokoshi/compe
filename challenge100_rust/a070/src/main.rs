fn main() {
    let a = read_line();
    let m = a[0] as i64;
    let n = a[1] as i64;
    let d = 1_000_000_007;
    println!("{}", f(m, n, d));
}

fn f(m: i64, n: i64, d: i64) -> i64 {
    if n == 1 {
        return m % d;
    }
    let half = f(m, n / 2, d);
    let even = (half * half) % d;
    return if n % 2 == 0 { even } else { (even * m) % d };
}

fn read_line() -> Vec<i32> {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).expect("");
    s.trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect()
}
