use proconio::input;

fn main() {
    input! {
        n: usize,
        h: usize,
        x: usize,
        mut p: [usize; n]
    }
    p.sort();
    for i in 0..n {
        if p[i] >= x - h {
            println!("{}", i + 1);
            return;
        }
    }
}
