fn main() {
    let mut v = vec![1, 2, 2, 2, 3, 3, 4];
    loop {
        println!("{:?}", v);
        if !next_permutation(&mut v) {
            break;
        }
    }

    let mut v = vec!['a', 'b', 'c', 'c', 'd'];
    loop {
        println!("{:?}", v);
        if !next_permutation(&mut v) {
            break;
        }
    }
}

fn next_permutation<T: Ord>(v: &mut Vec<T>) -> bool {
    let n = v.len();
    let mut i = 0;
    let mut j = 0;
    for x in 0..n - 1 {
        let x = n - 2 - x;
        if v[x] < v[x + 1] {
            i = x;
            break;
        }
        if x == 0 {
            return false;
        }
    }
    for x in 0..n {
        let x = n - 1 - x;
        if v[x] > v[i] {
            j = x;
            break;
        }
    }
    v.swap(i, j);
    v[i + 1..].reverse();

    return true;
}
