use proconio::input;

fn main() {
    input! {
        n: usize,
        xy: [(usize, usize); n]
    }
    let mut takahashi = 0;
    let mut aoki = 0;
    for i in 0..n {
        let (x, y) = xy[i];
        takahashi += x;
        aoki += y;
    }
    println!(
        "{}",
        if takahashi > aoki {
            "Takahashi"
        } else if takahashi < aoki {
            "Aoki"
        } else {
            "Draw"
        }
    )
}
