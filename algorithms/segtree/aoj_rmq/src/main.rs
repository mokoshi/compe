// https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=DSL_2_A&lang=jp
use my_rmq_segtree::MyRmqSegtree;

fn main() {
    // raw_impl();
    use_my_lib();
}

fn use_my_lib() {
    let l = read_line();
    let n = l[0] as usize;
    let q = l[1];

    let mut segtree = MyRmqSegtree::new(n);
    for _ in 0..q {
        let l = read_line();
        let cmd = l[0];
        let x = l[1];
        let y = l[2] as u64;
        if cmd == 0 {
            segtree.update(x as usize, y);
        } else {
            println!("{}", segtree.query(x as usize, y as usize + 1,))
        }
    }
}

const INF: u32 = (1 << 31) - 1;
fn raw_impl() {
    let l = read_line();
    let n = l[0];
    let q = l[1];

    // 数列の長さをカバーする最小の2^xを求める
    let ceil_log2 = 32 - (n - 1).leading_zeros();

    // 葉の数は2^x
    let leave_num = (1 << ceil_log2) as usize;

    let mut segtree = vec![INF; 2 * leave_num - 1];

    for _ in 0..q {
        let l = read_line();
        let cmd = l[0];
        let x = l[1];
        let y = l[2];
        if cmd == 0 {
            update(x as usize, y, leave_num, &mut segtree);
        } else {
            println!(
                "{}",
                find(
                    x as usize,
                    y as usize + 1,
                    0,
                    0,
                    leave_num,
                    leave_num,
                    &mut segtree
                )
            )
        }
    }
}

fn update(i: usize, x: u32, leave_num: usize, segtree: &mut Vec<u32>) {
    let mut i = leave_num - 1 + i;
    segtree[i] = x;
    while i > 0 {
        // 2分木の親のインデックス: (i-1)/2
        // 2分木の子のインデックス: i*2+1, i*2+2
        i = (i - 1) / 2;
        segtree[i] = segtree[i * 2 + 1].min(segtree[i * 2 + 2]);
    }
}
fn find(
    a: usize,
    b: usize,
    k: usize,
    l: usize,
    r: usize,
    leave_num: usize,
    segtree: &mut Vec<u32>,
) -> u32 {
    // println!("find: {}-{}, k:{}, l-r:{}-{}", a, b, k, l, r);
    if r <= a || l >= b {
        INF
    } else if a <= l && r <= b {
        segtree[k]
    } else {
        let lm = find(a, b, k * 2 + 1, l, (l + r) / 2, leave_num, segtree);
        let rm = find(a, b, k * 2 + 2, (l + r) / 2, r, leave_num, segtree);
        lm.min(rm)
    }
}

fn read_line() -> Vec<u32> {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).expect("");
    s.trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect()
}
