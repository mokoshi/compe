use proconio::input;

fn main() {
    input! {
        mm: usize,
        dd: usize,
        mut y: usize,
        mut m: usize,
        mut d: usize
    }
    if d == dd {
        d = 1;
        if m == mm {
            m = 1;
            y += 1;
        } else {
            m += 1;
        }
    } else {
        d += 1;
    }
    println!("{} {} {}", y, m, d);
}
