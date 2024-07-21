use im_rc::HashMap;
use proconio::input;

const MOD: u128 = 998244353;

fn main() {
    input! {
        n: u128
    }
    let mut t = HashMap::new();
    let mut t10 = HashMap::new();
    t.insert(1, n % MOD);
    t10.insert(1, 10);
    let keta = keta(n);
    println!("{}", calc(n, n, &mut t, &mut t10, keta));
}

fn keta(mut n: u128) -> u128 {
    let mut ret = 0;
    while n > 0 {
        n /= 10;
        ret += 1;
    }
    ret
}

fn calc(
    n: u128,
    cnt: u128,
    t: &mut HashMap<u128, u128>,
    t10: &mut HashMap<u128, u128>,
    keta: u128,
) -> u128 {
    if t.contains_key(&cnt) {
        return *t.get(&cnt).unwrap();
    }
    let half = cnt / 2;
    let (l, r) = if cnt % 2 == 0 {
        (calc(n, half, t, t10, keta), calc(n, half, t, t10, keta))
    } else {
        (calc(n, half, t, t10, keta), calc(n, half + 1, t, t10, keta))
    };
    let ret = (l + (r * calc10(keta * half, t10)) % MOD) % MOD;
    // println!("calc - n:{}, cnt:{}, ret:{}", n, cnt, ret);
    t.insert(cnt, ret);

    ret
}

fn calc10(zeros: u128, t10: &mut HashMap<u128, u128>) -> u128 {
    if t10.contains_key(&zeros) {
        return *t10.get(&zeros).unwrap();
    }
    let a = calc10(zeros / 2, t10);
    let ret = if zeros % 2 == 0 {
        a * a
    } else {
        ((a * a) % MOD) * 10
    } % MOD;
    // println!("calc10 - zeros:{}, ret:{}", zeros, ret);
    t10.insert(zeros, ret);

    ret
}

/*
 * d: n's digits
 *
 * A1 = mod(n)
 * A2 = mod(A1 + (mod(10^d) * A1))
 * A3 = mod(A2 + (mod(10^d)^2 * A2))
 * A3 = mod(A3 + (mod(10^d)^4 * A3))
 */
