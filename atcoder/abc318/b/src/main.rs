use proconio::input;

fn main() {
    input! {
        n: usize,
        abcd: [(usize, usize, usize, usize); n]
    }
    let mut ans = 0;
    for x in 0..100 {
        for y in 0..100 {
            for &(a, b, c, d) in &abcd {
                if x >= a && x < b && y >= c && y < d {
                    ans += 1;
                    break;
                }
            }
        }
    }
    println!("{}", ans);
}
