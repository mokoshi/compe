use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; k]
    }
    if k % 2 == 0 {
        let mut ans = 0;
        for i in 0..k / 2 {
            ans += a[i * 2 + 1] - a[i * 2];
        }
        println!("{}", ans);
    } else {
        let mut st = vec![0; k];
        let mut et = vec![0; k];
        for i in 1..k {
            if i % 2 == 1 {
                st[i] = st[i - 1] + a[i] - a[i - 1];
                et[i] = et[i - 1] + a[k - i] - a[k - i - 1];
            } else {
                st[i] = st[i - 1];
                et[i] = et[i - 1];
            }
        }
        let mut ans = i64::MAX;
        for i in 0..k {
            let c = st[i] + et[k - i - 1];
            // println!("remove {}, c = {}", i, c);
            ans = ans.min(c as i64);
        }
        // println!("{:?}, {:?}", st, et);
        println!("{}", ans);
    }
}
