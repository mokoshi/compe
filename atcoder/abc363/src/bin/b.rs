use proconio::input;

fn main() {
    input! {
        n: usize,
        t: i32,
        p: usize,
        mut l: [i32; n]
    }
    l.sort();
    l.reverse();

    println!("{}", (t - l[p - 1]).max(0));
}
