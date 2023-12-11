use proconio::input;

fn main() {
    input! {
        n: usize,
        l: i32,
        r: i32,
        a: [i32; n]
    }
    for &a in &a {
        print!(
            "{} ",
            if a < l {
                l
            } else if a > r {
                r
            } else {
                a
            }
        )
    }
}
