use proconio::input;

fn main() {
    input! { n: i32, s: i32, m: i32, l: i32 }

    let mut ans = 100000000;
    for sc in 0..=(n / 6 + 1) {
        let r = (n - sc * 6).max(0);
        for mc in 0..=(r as f32 / 8.0).ceil() as i32 {
            let r = r - mc * 8;
            let lc = (r as f32 / 12.0).ceil() as i32;
            ans = ans.min(sc * s + mc * m + lc * l);
        }
    }
    println!("{}", ans);
}
