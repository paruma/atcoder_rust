use cargo_snippet::snippet;

#[snippet(prefix = "use random_test::*;")]
/// ランダムなテストケースを生成するためのユーティリティモジュール
pub mod random_test {
    use std::{collections::HashSet, hash::Hash};

    use itertools::Itertools;
    use num::Integer;
    use num_integer::Roots;
    use petgraph::unionfind::UnionFind;
    use rand::Rng;

    /// 指定された個数のユニークな値を生成する。
    ///
    /// `gen` クロージャが返す値が `n` 種類に達するまで値の生成を繰り返す。
    ///
    /// # Arguments
    /// * `n` - 生成するユニークな値の個数
    /// * `gen` - 値を生成するクロージャ
    ///
    /// # Examples
    /// ```
    /// use atcoder_rust::mylib::random::random_test::*;
    /// use rand::{Rng, rngs::SmallRng, SeedableRng};
    ///
    /// let mut rng = SmallRng::from_entropy();
    /// let uniq_seq = generate_random_uniq_sequence(10, || rng.gen_range(0..100));
    /// assert_eq!(uniq_seq.len(), 10);
    /// ```
    pub fn generate_random_uniq_sequence<T, F>(n: usize, mut r#gen: F) -> Vec<T>
    where
        T: Hash + PartialEq + Eq,
        F: FnMut() -> T,
    {
        let mut set: HashSet<T> = HashSet::new();
        while set.len() != n {
            set.insert(r#gen());
        }

        set.into_iter().collect_vec()
    }

    /// 条件 `pred` を満たすランダムな値を生成する。
    ///
    /// `gen` クロージャで値を生成し、`pred` が `true` を返すまで繰り返す。
    ///
    /// # Arguments
    /// * `gen` - 値を生成するクロージャ
    /// * `pred` - 値が満たすべき条件を判定するクロージャ
    ///
    /// # Examples
    /// ```
    /// use atcoder_rust::mylib::random::random_test::*;
    /// use rand::{Rng, rngs::SmallRng, SeedableRng};
    ///
    /// let mut rng = SmallRng::from_entropy();
    /// let even_number = generate_random_while(|| rng.gen_range(0..100), |&x| x % 2 == 0);
    /// assert!(even_number % 2 == 0);
    /// ```
    pub fn generate_random_while<T, F, P>(mut r#gen: F, mut pred: P) -> T
    where
        F: FnMut() -> T,
        P: FnMut(&T) -> bool,
    {
        loop {
            let x = r#gen();
            if pred(&x) {
                return x;
            }
        }
    }

    /// `n_vertices` 頂点のランダムな木（辺のリスト）を生成する。
    ///
    /// # Arguments
    /// * `rng` - 乱数生成器
    /// * `n_vertices` - 木の頂点数
    ///
    /// # Examples
    /// ```
    /// use atcoder_rust::mylib::random::random_test::*;
    /// use rand::{Rng, rngs::SmallRng, SeedableRng};
    ///
    /// let mut rng = SmallRng::from_entropy();
    /// let tree = generate_random_tree(&mut rng, 5);
    /// assert_eq!(tree.len(), 4);
    /// ```
    pub fn generate_random_tree<R>(rng: &mut R, n_vertices: usize) -> Vec<(usize, usize)>
    where
        R: Rng,
    {
        let mut edges: Vec<(usize, usize)> = Vec::new();
        let mut uf: UnionFind<usize> = UnionFind::new(n_vertices);

        while edges.len() != n_vertices - 1 {
            let x = rng.gen_range(0..n_vertices);
            let y = rng.gen_range(0..n_vertices);
            if uf.union(x, y) {
                edges.push((x, y));
            }
        }
        edges
    }
    fn is_prime(n: i64) -> bool {
        if n <= 1 {
            return false;
        }
        for i in 2..=n.sqrt() {
            if n.is_multiple_of(&i) {
                return false;
            }
        }
        true
    }

    /// 指定された範囲 `[begin, end)` 内のランダムな素数を生成する。
    ///
    /// # Arguments
    /// * `rng` - 乱数生成器
    /// * `begin` - 範囲の下限（含む）
    /// * `end` - 範囲の上限（含まない）
    ///
    /// # Examples
    /// ```
    /// use atcoder_rust::mylib::random::random_test::*;
    /// use rand::{Rng, rngs::SmallRng, SeedableRng};
    ///
    /// let mut rng = SmallRng::from_entropy();
    /// let prime = generate_random_prime(&mut rng, 0, 100);
    /// ```
    pub fn generate_random_prime<R>(rng: &mut R, begin: i64, end: i64) -> i64
    where
        R: Rng,
    {
        let r#gen = || rng.gen_range(begin..end);
        generate_random_while(r#gen, |x| is_prime(*x))
    }
}

// test
#[cfg(test)]
mod tests {
    use super::random_test::*;
    use itertools::Itertools;

    #[test]
    fn test_generate_random_uniq_sequence() {
        use rand::{rngs::SmallRng, *};
        let mut rng = SmallRng::from_entropy();
        for _ in 0..10 {
            let xs = generate_random_uniq_sequence(10, || rng.gen_range(0..12));
            assert_eq!(xs.len(), 10);
            assert!(xs.iter().all_unique());
        }
    }

    #[test]
    fn test_generate_random_while() {
        use rand::{rngs::SmallRng, *};
        let mut rng = SmallRng::from_entropy();
        for _ in 0..10 {
            let x = generate_random_while(|| rng.gen_range(0..4), |&x| x % 2 == 0);
            assert!(x % 2 == 0 && (0..4).contains(&x));
        }
    }

    #[test]
    fn test_generate_random_tree() {
        use rand::{rngs::SmallRng, *};
        let mut rng = SmallRng::from_entropy();
        for _ in 0..10 {
            let n_vertices = 5;
            let edges = generate_random_tree(&mut rng, n_vertices);
            assert_eq!(edges.len(), n_vertices - 1);

            let is_connected = {
                let mut uf = ac_library::Dsu::new(n_vertices);
                for &(x, y) in &edges {
                    uf.merge(x, y);
                }
                (0..n_vertices).map(|v| uf.leader(v)).all_equal()
            };
            assert!(is_connected);
        }
    }

    #[test]
    fn test_generate_random_prime() {
        use rand::{rngs::SmallRng, *};
        let mut rng = SmallRng::from_entropy();
        for _ in 0..10 {
            let x = generate_random_prime(&mut rng, 0, 12);
            assert!((0..12).contains(&x));
            assert!(matches!(x, 2 | 3 | 5 | 7 | 11));
        }
    }
}
