use proconio::input;

fn main() {
    input! {
        n: usize,
        p: [i32; n],
    }

    let mut max = 0;
    for i in 1..n {
        max = max.max(p[i]);
    }
    println!("{}", (max - p[0] + 1).max(0));
}
