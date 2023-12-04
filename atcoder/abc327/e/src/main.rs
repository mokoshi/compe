use proconio::input;

fn main() {
    input! {
        n: usize,
        mut p: [i32; n]
    }
    p.sort();
    p.reverse();

    let mut r = 0_f64;
    for i in 0..n {
        let mut nr = 0_f64;
        let mut k = 1.0_f64;
        let mut d = 0_f64;
        for j in 0..=i {
            nr += k * (p[j] as f64);
            d += k;
            k *= 0.9;
        }
        println!("{}, {}", nr, d);
        nr = nr / d - (1200_f64 / ((i + 1) as f64).sqrt());
        println!("{}, {}", i, nr);
        if r < nr {
            r = nr;
        } else {
            // break;
        }
    }
    println!("{}", r);
}
