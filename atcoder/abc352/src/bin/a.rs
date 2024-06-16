use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
        y: usize,
        z: usize
    }
    if (x > y && x > z && z > y) || (x < y && x < z && z < y) {
        println!("Yes");
    } else {
        println!("No");
    }
}
