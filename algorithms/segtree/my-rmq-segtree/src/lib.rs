pub struct MyRmqSegtree {
    n: usize,
    size: usize,
    v: Vec<u64>,
}

impl MyRmqSegtree {
    pub fn new(n: usize) -> MyRmqSegtree {
        // 数列の長さをカバーする最小の2^xを求める
        let ceil_log2 = usize::BITS - (n - 1).leading_zeros();
        // 葉の数は2^x
        let size = (1 << ceil_log2) as usize;

        let v = vec![u64::MAX; 2 * size];
        Self { n, size, v }
    }

    pub fn update(&mut self, i: usize, x: u64) {
        // 末端から親に登って更新していくよ
        let mut i = self.size - 1 + i;
        self.v[i] = x;
        while i > 0 {
            // 2分木の親のインデックス: (i-1)/2
            // 2分木の子のインデックス: i*2+1, i*2+2
            i = (i - 1) / 2;
            self.v[i] = self.v[i * 2 + 1].min(self.v[i * 2 + 2]);
        }
    }

    pub fn query(&self, a: usize, b: usize) -> u64 {
        self.query_sub(a, b, 0, 0, self.size)
    }
    fn query_sub(&self, a: usize, b: usize, k: usize, l: usize, r: usize) -> u64 {
        if r <= a || l >= b {
            // 範囲外の場合
            u64::MAX
        } else if a <= l && r <= b {
            // 範囲に収まっている場合
            self.v[k]
        } else {
            let lm = self.query_sub(a, b, k * 2 + 1, l, (l + r) / 2);
            let rm = self.query_sub(a, b, k * 2 + 2, (l + r) / 2, r);
            lm.min(rm)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut segtree = MyRmqSegtree::new(5);
        segtree.update(0, 10);
        segtree.update(1, 20);
        segtree.update(2, 15);
        segtree.update(3, 12);
        segtree.update(4, 18);

        assert_eq!(segtree.query(0, 1), 10);
        assert_eq!(segtree.query(1, 2), 20);
        assert_eq!(segtree.query(2, 3), 15);
        assert_eq!(segtree.query(3, 4), 12);
        assert_eq!(segtree.query(4, 5), 18);

        assert_eq!(segtree.query(0, 2), 10);
        assert_eq!(segtree.query(1, 3), 15);
        assert_eq!(segtree.query(2, 4), 12);
        assert_eq!(segtree.query(3, 5), 12);

        assert_eq!(segtree.query(0, 3), 10);
        assert_eq!(segtree.query(1, 4), 12);
        assert_eq!(segtree.query(2, 5), 12);

        assert_eq!(segtree.query(0, 4), 10);
        assert_eq!(segtree.query(1, 5), 12);

        assert_eq!(segtree.query(0, 5), 10);
    }
}
