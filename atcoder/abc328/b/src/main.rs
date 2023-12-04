use proconio::input;

fn main() {
    input! {
        n: usize,
        d: [usize; n]
    }
    let mut ans = 0;
    for i in 1..=n {
        for j in 1..=d[i - 1] {
            for k in 1..=9 {
                if is_zoro(i, k) && is_zoro(j, k) {
                    ans += 1;
                    break;
                }
            }
        }
    }
    println!("{}", ans);
}

fn is_zoro(a: usize, d: usize) -> bool {
    let d1 = a % 10;
    let d2 = (a / 10) % 10;
    return d1 == d && (d2 == 0 || d2 == d);
}
