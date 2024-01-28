use proconio::input;

fn main() {
    input! {
        n: usize,
        q: [usize; n],
        a: [usize; n],
        b: [usize; n],
    }

    let mut anum = 10_000_000_000;
    for i in 0..n {
        if a[i] == 0 {
            continue;
        }
        anum = anum.min(q[i] / a[i]);
    }

    let mut ans = 0;
    for an in 0..=anum {
        let mut qq = q.clone();
        for i in 0..n {
            qq[i] -= an * a[i];
        }

        let mut bnum = 10_000_000_000;
        for i in 0..n {
            if b[i] == 0 {
                continue;
            }
            bnum = bnum.min(qq[i] / b[i]);
        }

        ans = ans.max(an + bnum);
    }
    println!("{}", ans);
}
