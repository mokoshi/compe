use proconio::input;

fn main() {
    input! {
        n: usize,
        m: f64,
    }
    let mut c = vec![0.0; n];
    let mut p = vec![0; n];
    let mut e = vec![0.0; n];
    for i in 0..n {
        input! {
            ci: f64,
            pi: usize,
            si: [f64; pi],
        }
        c[i] = ci;
        p[i] = pi;
        e[i] = si.iter().sum::<f64>() / si.len() as f64 / ci as f64;
    }

    let mut indexes: Vec<usize> = (0..e.len()).collect();
    indexes.sort_by(|&i, &j| e[i].partial_cmp(&e[j]).unwrap());

    let mut ans = 0.0;
    let mut em = 0.0;
    for i in 0..n {
        let cnt = ((m - em) / e[i]) as i32;
        em += e[i] * c[i] * cnt as f64;
        ans += cnt as f64 * c[i];
    }
    println!("{}", ans);

    let remain = m - em;
    let mut indexes: Vec<usize> = (0..c.len()).collect();
    indexes.sort_by(|&i, &j| c[i].partial_cmp(&c[j]).unwrap());
    ans += remain / e[indexes[0]] * c[indexes[0]];

    println!("{}", ans);
}
