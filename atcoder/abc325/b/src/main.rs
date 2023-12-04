use proconio::input;

fn main() {
    input! {
        n: usize,
        wx: [(i64, i64); n]
    }
    let mut ans = 0;
    for i in 0..24 {
        let mut total = 0;
        for (w, x) in &wx {
            let t = (i + x) % 24;
            if t >= 9 && t + 1 <= 18 {
                total += w;
            }
        }
        ans = ans.max(total)
    }
    println!("{}", ans);
}
