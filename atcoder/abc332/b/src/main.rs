use proconio::input;

fn main() {
    input! {
        k: u32,
        g: u32,
        m: u32
    }
    let mut gg = 0;
    let mut mm = 0;
    for _ in 0..k {
        if gg == g {
            gg = 0;
        } else if mm == 0 {
            mm = m;
        } else {
            if g - gg > mm {
                gg += mm;
                mm = 0;
            } else {
                mm -= g - gg;
                gg = g;
            }
        }
    }
    println!("{} {}", gg, mm)
}
