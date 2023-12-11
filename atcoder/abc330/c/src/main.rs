use proconio::input;

fn main() {
    input! {
        d: i64
    }
    let m = ((2 * 10_i64.pow(12)) as f64).sqrt().ceil() as i64;
    let mut ans = d;
    for x in 1..=m {
        let r = (d - x * x).abs();
        let y = (r as f64).sqrt().floor() as i64;
        ans = ans.min((x * x + y * y - d).abs());
        ans = ans.min((x * x + (y + 1) * (y + 1) - d).abs());
    }
    println!("{}", ans)
}
