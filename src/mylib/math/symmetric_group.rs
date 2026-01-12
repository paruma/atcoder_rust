use cargo_snippet::snippet;
#[allow(clippy::module_inception)]
#[snippet(prefix = "use symmetric_group::*;")]
pub mod symmetric_group {
    /// 置換を巡回置換の積で表したときの巡回置換のリストを返す。
    ///
    /// 例: `make_cycles(&[1, 0, 3, 2]) == vec![vec![0, 1], vec![2, 3]]`
    ///
    /// # 計算量
    /// O(N)
    pub fn make_cycles(ps: &[usize]) -> Vec<Vec<usize>> {
        let n = ps.len();
        let mut visited = vec![false; n];

        let mut cycles = vec![];

        for init in 0..n {
            if visited[init] {
                continue;
            }
            let mut cycle = vec![];
            let mut cur = init;
            while !visited[cur] {
                cycle.push(cur);
                visited[cur] = true;
                cur = ps[cur];
            }
            cycles.push(cycle);
        }
        cycles
    }

    /// 置換の積を計算する
    ///
    /// # 計算量
    /// O(N)
    pub fn mul_of_permutation(ps: &[usize], mut qs: Vec<usize>) -> Vec<usize> {
        // ret[i] = ps[qs[i]] となる ret を返す
        for q in qs.iter_mut() {
            *q = ps[*q];
        }
        qs
    }

    /// 置換の `k` 乗を計算する
    ///
    /// # 計算量
    /// O(N)
    pub fn pow_of_permutation(ps: &[usize], k: u64) -> Vec<usize> {
        let n = ps.len();
        let mut ret = vec![0; n];
        let cycles = make_cycles(ps);
        for cycle in cycles {
            let len = cycle.len() as u64;
            let k = (k % len) as usize;
            for (i, &x) in cycle.iter().enumerate() {
                ret[x] = cycle[(i + k) % cycle.len()];
            }
        }
        ret
    }

    /// 逆置換を計算する
    ///
    /// # 計算量
    /// O(N)
    pub fn inv_of_permutation(ps: &[usize]) -> Vec<usize> {
        let n = ps.len();
        let mut ret = vec![0; n];
        for (i, &p) in ps.iter().enumerate() {
            ret[p] = i;
        }
        ret
    }

    /// 転倒数を計算する
    ///
    /// # 計算量
    /// O(N log N)
    pub fn inversion_number(ps: &[usize]) -> i64 {
        use ac_library::FenwickTree;
        // 転倒数を計算する。acl の fenwick tree を使う
        let n = ps.len();
        let mut ft = FenwickTree::new(n, 0_i64);
        let mut ans = 0;
        for &p in ps {
            ans += ft.sum(p + 1..n);
            ft.add(p, 1);
        }
        ans
    }
}

#[cfg(test)]
mod test_symmetric_group {
    use crate::math::symmetric_group::symmetric_group::{
        inv_of_permutation, inversion_number, make_cycles, mul_of_permutation, pow_of_permutation,
    };

    #[test]
    fn test_make_cycles() {
        // 恒等置換
        assert_eq!(make_cycles(&[] as &[usize]), vec![] as Vec<Vec<usize>>);
        assert_eq!(make_cycles(&[0]), vec![vec![0]]);
        assert_eq!(make_cycles(&[0, 1, 2]), vec![vec![0], vec![1], vec![2]]);

        // 単一の巡回置換
        assert_eq!(make_cycles(&[1, 2, 0]), vec![vec![0, 1, 2]]);
        assert_eq!(make_cycles(&[1, 0]), vec![vec![0, 1]]);

        // 複数の巡回置換
        assert_eq!(make_cycles(&[1, 0, 3, 2]), vec![vec![0, 1], vec![2, 3]]);
        assert_eq!(make_cycles(&[2, 3, 0, 1]), vec![vec![0, 2], vec![1, 3]]);
    }

    #[test]
    fn test_mul_of_permutation() {
        let p = vec![1, 2, 0];
        let q = vec![0, 1, 2];
        assert_eq!(mul_of_permutation(&p, q), vec![1, 2, 0]);

        let p = vec![1, 2, 0];
        let q = vec![1, 0, 2];
        // q[0]=1 -> p[1]=2
        // q[1]=0 -> p[0]=1
        // q[2]=2 -> p[2]=0
        assert_eq!(mul_of_permutation(&p, q), vec![2, 1, 0]);
    }

    #[test]
    fn test_pow_of_permutation() {
        let p = vec![1, 2, 0];
        assert_eq!(pow_of_permutation(&p, 0), vec![0, 1, 2]);
        assert_eq!(pow_of_permutation(&p, 1), vec![1, 2, 0]);
        assert_eq!(pow_of_permutation(&p, 2), vec![2, 0, 1]);
        assert_eq!(pow_of_permutation(&p, 3), vec![0, 1, 2]);
        assert_eq!(pow_of_permutation(&p, 100), vec![1, 2, 0]);
    }

    #[test]
    fn test_inv_of_permutation() {
        let p = vec![1, 2, 0];
        assert_eq!(inv_of_permutation(&p), vec![2, 0, 1]);
    }

    #[test]
    fn test_inversion_number() {
        assert_eq!(inversion_number(&[0, 1, 2]), 0);
        assert_eq!(inversion_number(&[2, 1, 0]), 3); // (2,1), (2,0), (1,0)
        assert_eq!(inversion_number(&[1, 0, 2]), 1); // (1,0)
    }
}
