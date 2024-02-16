pub trait Monoid {
    type S: Clone + Copy;
    // 単位元
    fn identity_element() -> Self::S;
    // 二項演算
    fn binary_operation(a: Self::S, b: Self::S) -> Self::S;
}
pub struct MinMonoid {}
impl Monoid for MinMonoid {
    type S = u64;
    fn identity_element() -> Self::S {
        u64::MAX
    }
    fn binary_operation(a: Self::S, b: Self::S) -> Self::S {
        std::cmp::min(a, b)
    }
}

pub struct MySegtree<M: Monoid> {
    n: usize,
    size: usize,
    tree: Vec<M::S>,
}
impl<M: Monoid> MySegtree<M> {
    pub fn new(n: usize) -> MySegtree<M> {
        // 数列の長さをカバーする最小の2^xを求める
        let ceil_log2 = usize::BITS - (n - 1).leading_zeros();
        // 葉の数は2^x
        let size = (1 << ceil_log2) as usize;

        let tree = vec![M::identity_element(); 2 * size];
        Self { n, size, tree }
    }

    pub fn update(&mut self, i: usize, x: M::S) {
        // 末端から親に登って更新していくよ
        let mut i = self.size - 1 + i;
        self.tree[i] = x;
        while i > 0 {
            // 2分木の親のインデックス: (i-1)/2
            // 2分木の子のインデックス: i*2+1, i*2+2
            i = (i - 1) / 2;
            self.tree[i] = M::binary_operation(self.tree[i * 2 + 1], self.tree[i * 2 + 2]);
        }
    }

    pub fn query(&self, a: usize, b: usize) -> M::S {
        self.query_sub(a, b, 0, 0, self.size)
    }
    fn query_sub(&self, a: usize, b: usize, k: usize, l: usize, r: usize) -> M::S {
        if r <= a || l >= b {
            // 範囲外の場合
            M::identity_element()
        } else if a <= l && r <= b {
            // 範囲に収まっている場合
            self.tree[k]
        } else {
            let lm = self.query_sub(a, b, k * 2 + 1, l, (l + r) / 2);
            let rm = self.query_sub(a, b, k * 2 + 2, (l + r) / 2, r);
            M::binary_operation(lm, rm)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn min() {
        let mut segtree = MySegtree::<MinMonoid>::new(5);
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
