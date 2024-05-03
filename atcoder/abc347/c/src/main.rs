use proconio::input;

fn main() {
    input! {
        n: usize,
        a: u64,
        b: u64,
        d: [u64; n]
    }
    if n == 1 {
        println!("Yes");
        return;
    }
    let mut max = 0;
    for i in 1..n {
        let c = (d[i] - d[0]) % (a + b);
        println!("{}", c);
        max = max.max(c);
    }
    println!("{}", if max < a { "Yes" } else { "No" });
}

/*
3 2 3
1 2 7

3 2 3
2 3 8

3 2 3
3 4 9

3 2 3
4 5 10

3 2 3
5 6 11

 */
