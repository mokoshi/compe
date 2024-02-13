use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n]
    }
    let mut l = 1;
    let mut r = (n + 1) / 2;
    while l < r {
        let m = (l + r + 1) / 2;
        // println!("l: {}, r: {}, check: {}", l, r, m);
        if check(&a, m) {
            // println!("ok. l = {}", m);
            l = m;
        } else {
            // println!("ng. r = {}", m - 1);
            r = m - 1;
        }
    }
    println!("{}", l);
}

fn check(a: &Vec<usize>, m: usize) -> bool {
    'root: for offset in 0..=a.len() - (2 * m - 1) {
        // println!("offset: {}", offset);
        for i in 0..m {
            // println!("check a[{}] < {}", i + offset, i + 1);
            if a[i + offset] < i + 1 {
                // println!("over, continue");
                continue 'root;
            }
        }
        for i in m..2 * m - 1 {
            // println!("check a[{}] < {}", i + offset, 2 * m - 1 - i);
            if a[i + offset] < 2 * m - 1 - i {
                // println!("over, continue");
                continue 'root;
            }
        }
        return true;
    }
    return false;
}

/*
6
1 2 2 3 3 2

 */
