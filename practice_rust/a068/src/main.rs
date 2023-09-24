fn main() {
    let mut n = read_line()[0];
    let m = (n as f64).sqrt().round() as i32;

    print!("{}:", n);

    let mut c = 2;
    while c <= m {
        if n % c == 0 {
            n /= c;
            print!(" {}", c);
        } else {
            c += 1;
        }
    }
    if n > 2 {
        print!(" {}", n);
    }
    println!();
}

fn read_line() -> Vec<i32> {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).expect("");
    s.trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect()
}
