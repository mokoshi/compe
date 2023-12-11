use proconio::input;

fn main() {
    input! {n: usize, m: usize, a: [usize; m]}

    let mut table = vec![0; n + 1];
    let mut prev = (n + 1, 0);
    for a in a {
        table[a] += 1;
        if prev.1 <= table[a] {
            let newa = if prev.1 < table[a] || a < prev.0 {
                a
            } else {
                prev.0
            };
            prev = (newa, table[a]);
            println!("{}", newa);
        } else {
            println!("{}", prev.0);
        }
    }
}
