use proconio::input;

fn main() {
    input! {
        n: usize,
        p: [usize; n],
        q: usize,
        ab: [(usize, usize); q]
    }
    for &(a, b) in &ab {
        let pa = p.iter().position(|&x| x == a).unwrap();
        let pb = p.iter().position(|&x| x == b).unwrap();
        println!("{}", if pa < pb { a } else { b });
    }
}
