use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars,
        t: Chars
    }
    let mut table = vec![vec![]; 256];
    for i in 0..s.len() {
        table[s[i] as usize].push(i);
    }
}

fn is_substring(table: &Vec<Vec<usize>>, n: usize, t: &Vec<char>, k: usize) -> bool {
    let mut a = 0;
    let mut p = 0;

    let c = t[0];
    let table = &table[c as usize];
    let cnt = table.len();
    a += k / cnt;
    if k % cnt == 0 {
        p = 0;
    } else {
        a += 1;
        p = table.partition_point(|&x| x > p);
    }

    false
}
