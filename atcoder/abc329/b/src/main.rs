use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [usize; n]
    }
    a.sort();
    a.reverse();

    let m = a[0];
    for i in 1..n {
        if a[i] != m {
            println!("{}", a[i]);
            return;
        }
    }
}
