use proconio::input;

fn main() {
    input! {
        n:usize,
        m: usize,
        cn: [(f64, f64, f64); n],
        cm: [(f64, f64); m]
    }

    let mut mr = vec![std::f64::MAX; m];
    for i in 0..m {
        for j in i + 1..m {
            let r = ((cm[i].0 - cm[j].0).powi(2) + (cm[i].1 - cm[j].1).powi(2)).sqrt() / 2.0;
            mr[i] = mr[i].min(r);
            mr[j] = mr[j].min(r);
        }
        for j in 0..n {
            let r = ((cm[i].0 - cn[j].0).powi(2) + (cm[i].1 - cn[j].1).powi(2)).sqrt() - cn[j].2;
            mr[i] = mr[i].min(r);
        }
    }

    let mut ans = std::f64::MAX;
    for i in 0..m {
        ans = ans.min(mr[i]);
    }
    for i in 0..n {
        ans = ans.min(cn[i].2);
    }
    println!("{}", ans);
}
