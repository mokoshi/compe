use num::Integer;
use proconio::input;

fn main() {
    input! {
        n: u64
    }
    let d = 10_u64.pow(Integer::div_ceil(&n.to_string().len(), &2) as u32);
    for i in 1..=d {
        let a = 10_u64.pow(i.to_string().len() as u32);
        let k = i + i * a;
        if k > n {
            println!("{}", i - 1);
            break;
        }
    }
}
