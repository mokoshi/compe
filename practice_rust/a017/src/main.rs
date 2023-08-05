use std::{
    collections::{HashMap, HashSet},
    io,
};

mod perm {
    pub struct PermutationIterator<T: Ord + Clone> {
        li: Vec<T>,
        is_finished: bool,
    }

    impl<T: Ord + Clone> PermutationIterator<T> {
        pub fn new(mut li: Vec<T>) -> PermutationIterator<T> {
            let is_finished = li.is_empty();
            li.sort();
            PermutationIterator { li, is_finished }
        }
    }

    impl<T: Ord + Clone> Iterator for PermutationIterator<T> {
        type Item = Vec<T>;

        // C++ の next_permutation 実装をもとに
        // ref. https://cpprefjp.github.io/reference/algorithm/next_permutation.html
        fn next(&mut self) -> Option<Self::Item> {
            if self.is_finished {
                return None;
            }

            if self.li.len() == 1 {
                self.is_finished = true;
                return Some(self.li.clone());
            }

            let mut i = self.li.len() - 1;
            let res = self.li.clone();

            loop {
                let ii = i;
                i -= 1;
                if self.li[i] < self.li[ii] {
                    let mut j = self.li.len() - 1;
                    while self.li[i] >= self.li[j] {
                        j -= 1;
                    }

                    self.li.swap(i, j);
                    self.li[ii..].reverse();
                    return Some(res);
                }
                if i == 0 {
                    self.li.reverse();
                    self.is_finished = true;
                    return Some(res);
                }
            }
        }
    }

    pub trait Permutation<T: Ord + Clone> {
        fn permutation(self) -> PermutationIterator<T>;
    }

    // Vec<T> に対してのみの実装する
    // impl <T: Ord + Clone> Permutation<T> for Vec<T> {
    //     fn permutation(self) -> PermutationIterator<T> {
    //         PermutationIterator::new(self)
    //     }
    // }

    // IntoIterator を実装するものに対して Permutation を実装する
    impl<T: Ord + Clone, I: IntoIterator<Item = T>> Permutation<T> for I {
        fn permutation(self) -> PermutationIterator<T> {
            PermutationIterator::new(self.into_iter().collect())
        }
    }
}

// https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ALDS1_13_A&lang=ja
fn main() {
    use perm::Permutation;

    let (_k, rc) = input();

    let mut r2c = HashMap::new();
    for (r, c) in rc {
        r2c.insert(r, c);
    }

    'root: for cc in vec![0, 1, 2, 3, 4, 5, 6, 7].permutation() {
        let mut ru = HashSet::new();
        let mut rd = HashSet::new();
        for (i, &c) in cc.iter().enumerate() {
            let r = i as i32;
            let ac = r2c.get(&r);
            if ac != None && ac.unwrap() != &c {
                continue 'root;
            }

            let ruv = c + r;
            let rdv = c - r;
            if ru.contains(&ruv) || rd.contains(&rdv) {
                continue 'root;
            }
            ru.insert(ruv);
            rd.insert(rdv);
        }

        for c in cc {
            println!(
                "{}",
                (0..8)
                    .map(|x| if x == c { 'Q' } else { '.' })
                    .collect::<String>()
            );
        }
        break;
    }
}

fn input() -> (i32, Vec<(i32, i32)>) {
    let k: i32 = read_line()[0];

    let mut rc = Vec::<(i32, i32)>::new();
    for _ in 0..k {
        let l = read_line();
        rc.push((l[0], l[1]));
    }

    (k, rc)
}

fn read_line() -> Vec<i32> {
    let mut s = String::new();
    io::stdin().read_line(&mut s).expect("");
    s.trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect()
}
