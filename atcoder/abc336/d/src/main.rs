use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n]
    }
    let mut l = vec![0; n];
    let mut r = vec![0; n];
    let mut lm = 1;
    let mut rm = 1;
    for i in 0..n {
        l[i] = lm.min(a[i]);
        lm = l[i] + 1;
    }
    for i in 0..n {
        r[n - 1 - i] = rm.min(a[n - 1 - i]);
        rm = r[n - 1 - i] + 1;
    }
    let mut ans = 1;
    for i in 0..n {
        ans = ans.max(l[i].min(r[i]));
    }
    println!("{}", ans);
}

/*
6
1 2 2 3 3 2

 */
