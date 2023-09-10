use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        s: String,
        c: [usize; n],
    }

    let schars: Vec<char> = s.chars().collect();
    let mut mm: Vec<Vec<char>> = vec![vec![]; m];
    let mut idx: Vec<usize> = vec![];
    for i in 0..n {
        let mi = c[i] - 1;
        mm[mi].push(schars[i]);
        idx.push(mi);
    }
    let mut mmi: Vec<usize> = vec![0; m];
    for i in idx {
        let l = mm[i].len();
        print!("{}", mm[i][(mmi[i] + l - 1) % l]);
        mmi[i] += 1;
    }

    println!();
}
