use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize
    }
    let mut parts = vec![(0 as i64, 0 as i64); n];
    for i in 0..n {
        parts[i] = (i as i64 + 1, 0);
    }
    let mut mvcnt = 0;
    for i in 0..q {
        input! { a: u64 }
        if a == 1 {
            input! { b: char }
            // println!("-------------- move");
            let head = parts[(n - mvcnt % n) % n];
            // println!("head: {:?}, move: {}", head, b);
            parts[n - (mvcnt % n) - 1] = match b {
                'L' => (head.0 - 1, head.1),
                'R' => (head.0 + 1, head.1),
                'U' => (head.0, head.1 + 1),
                'D' => (head.0, head.1 - 1),
                _ => (head.0, head.1),
            };
            // println!("{:?}", parts);
            mvcnt += 1;
        } else {
            input! { p: usize }
            // println!("-------------- print");
            let pos = (n - mvcnt % n + (p - 1)) % n;
            // let head = (n - mvcnt % n) % n;
            // println!("{:?}", parts);
            // println!(
            //     "p: {}, head: {}, pos: {}, parts: {:?}",
            //     p, head, pos, parts[pos]
            // );
            println!("{} {}", parts[pos].0, parts[pos].1);
        }
    }
}

/*
10 5
2 1
1 U
2 1
1 U
2 1

 */

/*
10 12
1 U
1 R
1 D
1 L
1 U
1 R
1 D
1 L
1 U
1 R
1 D
1 L

*/
