use proconio::input;

fn main() {
    input! {
        a: [u32; 9],
        b: [u32; 8],
    }
    let at = a.iter().sum::<u32>();
    let bt = b.iter().sum::<u32>();
    println!("{}", at - bt + 1);
}
