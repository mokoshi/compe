use std::{collections::HashSet, io};

// https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ALDS1_4_B&lang=ja
fn main() {
    let (_n, s, _q, t) = input();

    let mut found = HashSet::new();
    for target in t {
        if binary_search(&s, target).is_some() {
            found.insert(target);
        }
    }

    println!("{}", found.len());
}

fn binary_search(list: &Vec<i32>, target: i32) -> Option<usize> {
    let mut left = 0;
    let mut right = list.len() - 1;

    while left <= right {
        let mid = (left + right) / 2;
        if list[mid] == target {
            return Some(mid);
        } else if list[mid] < target {
            left = mid + 1;
        } else {
            right = mid - 1;
        }
    }

    return None;
}

fn input() -> (i32, Vec<i32>, i32, Vec<i32>) {
    let n = read_line()[0];
    let s = read_line();
    let q = read_line()[0];
    let t = read_line();

    (n, s, q, t)
}

fn read_line() -> Vec<i32> {
    let mut s = String::new();
    io::stdin().read_line(&mut s).expect("");
    s.trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect()
}
