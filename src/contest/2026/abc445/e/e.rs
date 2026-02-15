// #[fastout]
fn main() {
    input! {
        t: usize
    }

    use ac_library::ModInt998244353 as Mint;

    let sieve = EratosthenesSieve::new(10_000_000);

    for _ in 0..t {
        input! {
            n: usize,
            xs: [usize; n],
        }

        let xs_f = xs
            .iter()
            .copied()
            .map(|x| sieve.prime_factorize(x))
            .collect_vec();

        // 変化する部分は一部だから差分管理できそう？

        // {{ . }} を多重集合を表すとするとき、
        // prime_to_exps[p] = {{x の素因数 p の指数 | x in xs}}
        let mut prime_to_exps = HashMap::<usize, BTreeMultiSet<usize>>::new();

        let mut lcm = Mint::new(1);

        for i in (0..n).rev() {
            // dbg!(lcm);
            for &(p, exp) in &xs_f[i] {
                let max_exp = prime_to_exps
                    .get(&p)
                    .and_then(|bag| bag.max())
                    .copied()
                    .unwrap_or(0);
                if exp > max_exp {
                    lcm *= Mint::new(p).pow((exp - max_exp) as u64);
                }
                prime_to_exps.entry(p).or_default().insert(exp);
            }
        }

        // dbg!(lcm);
        let mut ans = vec![];

        for i in 0..n {
            // right から i を取る
            // dbg!(lcm);
            for &(p, exp) in &xs_f[i] {
                prime_to_exps.entry(p).or_default().remove1(&exp);
                let max_exp = prime_to_exps
                    .get(&p)
                    .and_then(|bag| bag.max())
                    .copied()
                    .unwrap_or(0);
                if exp > max_exp {
                    lcm /= Mint::new(p).pow((exp - max_exp) as u64);
                }
            }
            // dbg!(lcm);

            ans.push(lcm);
            // left に i を追加する

            for &(p, exp) in &xs_f[i] {
                let max_exp = prime_to_exps
                    .get(&p)
                    .and_then(|bag| bag.max())
                    .copied()
                    .unwrap_or(0);
                if exp > max_exp {
                    lcm *= Mint::new(p).pow((exp - max_exp) as u64);
                }
                prime_to_exps.entry(p).or_default().insert(exp);
            }
        }
        print_vec_1line(&ans);
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;
    #[allow(unused_imports)]
    use rand::{rngs::SmallRng, seq::SliceRandom, *};

    #[test]
    fn test_problem() {
        assert_eq!(1 + 1, 2);
    }

    /// 間違っていたら false を返す
    fn process_one_test(rng: &mut SmallRng) -> bool {
        // ==== 問題を作る ====
        let n = rng.random_range(1..=10);
        let xs = (0..n).map(|_| rng.random_range(0..10)).collect_vec();

        // ==== 解く ====
        let main_ans = xs.len();
        let naive_ans = 1;

        // ==== 間違っていたら報告をする ====
        if main_ans != naive_ans {
            // 問題を出力
            println!("{:?}", (n, xs));
            println!("main ans : {:?}", main_ans);
            println!("naive ans: {:?}", naive_ans);
            return false;
        }
        true
    }

    #[allow(unreachable_code)]
    #[test]
    #[ignore]
    fn test_with_naive() {
        let num_tests = 100;
        let max_wrong_case = 10; // この件数間違いが見つかったら打ち切り
        let mut cnt_wrong = 0;
        let mut rng = SmallRng::seed_from_u64(42);
        // let mut rng = SmallRng::from_os_rng();
        for _ in 0..num_tests {
            let is_ok = process_one_test(&mut rng);
            if !is_ok {
                cnt_wrong += 1;
            }
            if cnt_wrong >= max_wrong_case {
                break;
            }
        }
        if cnt_wrong > 0 {
            println!("{} cases are wrong.", cnt_wrong);
            panic!();
        }
    }
}

// ====== import ======
#[allow(unused_imports)]
use {
    itertools::{Itertools, chain, iproduct, izip},
    proconio::{
        derive_readable, fastout, input,
        marker::{Bytes, Chars, Usize1},
    },
    rand::{Rng, SeedableRng, rngs::SmallRng, seq::SliceRandom},
    std::{
        cmp::Reverse,
        collections::{BinaryHeap, HashMap, HashSet},
    },
};

// ====== output func ======
#[allow(unused_imports)]
use print_util::*;
pub mod print_util {
    use itertools::Itertools;
    use proconio::fastout;
    #[fastout]
    pub fn print_vec<T: std::fmt::Display>(arr: &[T]) {
        for a in arr {
            println!("{}", a);
        }
    }
    #[fastout]
    pub fn print_vec_1line<T: std::fmt::Display>(arr: &[T]) {
        let msg = arr.iter().map(|x| format!("{}", x)).join(" ");
        println!("{}", msg);
    }
    #[fastout]
    pub fn print_vec2<T: std::fmt::Display>(arr: &Vec<Vec<T>>) {
        for row in arr {
            let msg = row.iter().map(|x| format!("{}", x)).join(" ");
            println!("{}", msg);
        }
    }
    pub fn print_bytes(bytes: &[u8]) {
        let msg = String::from_utf8(bytes.to_vec()).unwrap();
        println!("{}", msg);
    }
    pub fn print_chars(chars: &[char]) {
        let msg = chars.iter().collect::<String>();
        println!("{}", msg);
    }
    #[fastout]
    pub fn print_vec_bytes(vec_bytes: &[Vec<u8>]) {
        for row in vec_bytes {
            let msg = String::from_utf8(row.to_vec()).unwrap();
            println!("{}", msg);
        }
    }
    #[fastout]
    pub fn print_vec_chars(vec_chars: &[Vec<char>]) {
        for row in vec_chars {
            let msg = row.iter().collect::<String>();
            println!("{}", msg);
        }
    }
    pub fn print_yesno(ans: bool) {
        let msg = if ans { "Yes" } else { "No" };
        println!("{}", msg);
    }
}

// ====== snippet ======
use eratosthenes_sieve::*;
pub mod eratosthenes_sieve {
    #[derive(Clone, Debug)]
    pub struct EratosthenesSieve {
        is_prime_list: Vec<bool>,
        min_factor_list: Vec<usize>,
    }
    impl EratosthenesSieve {
        /// [0, n] の区間でエラトステネスのふるいをする
        /// # 計算量
        /// O(n log(log(n)))
        pub fn new(n: usize) -> Self {
            let mut is_prime_list = vec![true; n + 1];
            let mut min_factor_list = vec![0; n + 1];
            is_prime_list[0] = false;
            is_prime_list[1] = false;
            for p in 2..=n {
                if !is_prime_list[p] {
                    continue;
                }
                min_factor_list[p] = p;
                for q in (p * 2..=n).step_by(p) {
                    is_prime_list[q] = false;
                    if min_factor_list[q] == 0 {
                        min_factor_list[q] = p;
                    }
                }
            }
            Self {
                is_prime_list,
                min_factor_list,
            }
        }
        /// n が素数かどうか判定する
        /// # 計算量
        /// O(1)
        pub fn is_prime(&self, n: usize) -> bool {
            self.is_prime_list[n]
        }
        /// n を素因数分解する。key を素数、value をその素数の指数としたペアのリストを返す。
        /// # 計算量
        /// O(log n)
        pub fn prime_factorize(&self, n: usize) -> Vec<(usize, usize)> {
            let mut n = n;
            let mut res = Vec::new();
            while n > 1 {
                let p = self.min_factor_list[n];
                let mut exp = 0;
                while self.min_factor_list[n] == p {
                    n /= p;
                    exp += 1;
                }
                res.push((p, exp));
            }
            res
        }
        /// n の正の約数を列挙する
        /// # 計算量
        /// O(nの約数の個数)
        pub fn divisors(&self, n: usize) -> Vec<usize> {
            let mut res = vec![1];
            let pf = self.prime_factorize(n);
            for (p, e) in pf {
                let n = res.len();
                for i in 0..n {
                    let mut tmp = 1;
                    for _ in 0..e {
                        tmp *= p;
                        res.push(res[i] * tmp);
                    }
                }
            }
            res
        }
    }
}
use btree_multiset::*;
#[allow(clippy::module_inception)]
pub mod btree_multiset {
    use std::{
        borrow::Borrow,
        collections::{BTreeMap, btree_map::Range},
        ops::RangeBounds,
    };
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct BTreeMultiSet<T> {
        map: BTreeMap<T, usize>,
        length: usize,
    }
    impl<T> Default for BTreeMultiSet<T> {
        fn default() -> Self {
            Self::new()
        }
    }
    impl<T> BTreeMultiSet<T> {
        pub const fn new() -> BTreeMultiSet<T> {
            BTreeMultiSet {
                map: BTreeMap::new(),
                length: 0,
            }
        }
        pub fn range<R>(&self, range: R) -> Range<'_, T, usize>
        where
            T: Ord,
            R: RangeBounds<T>,
        {
            self.map.range(range)
        }
        /// 内部の BTreeMap のイテレータを返す。
        /// 要素とその個数のペア `(&T, &usize)` を巡回する。
        pub fn iter(&self) -> std::collections::btree_map::Iter<'_, T, usize> {
            self.map.iter()
        }
        /// 最小の要素を返す。
        /// 空の場合は `None` を返す。計算量は $O(\log K)$ ($K$ は要素の種類数)。
        pub fn min(&self) -> Option<&T>
        where
            T: Ord,
        {
            self.map.first_key_value().map(|(k, _)| k)
        }
        /// 最大の要素を返す。
        /// 空の場合は `None` を返す。計算量は $O(\log K)$ ($K$ は要素の種類数)。
        pub fn max(&self) -> Option<&T>
        where
            T: Ord,
        {
            self.map.last_key_value().map(|(k, _)| k)
        }
        /// 重複を考慮して、$n$ 番目に小さい要素を返す（0-indexed）。
        /// $n$ が全体の要素数（`len()`）以上の場合は `None` を返す。
        /// 計算量は $O(\log K + \min(m, K))$ ($m$ は走査したユニークな要素数、$K$ は種類数)。
        pub fn nth_min(&self, n: usize) -> Option<&T>
        where
            T: Ord,
        {
            let mut sum = 0;
            for (val, &cnt) in self.iter() {
                if sum + cnt > n {
                    return Some(val);
                }
                sum += cnt;
            }
            None
        }
        /// 重複を考慮して、$n$ 番目に大きい要素を返す（0-indexed）。
        /// $n$ が全体の要素数（`len()`）以上の場合は `None` を返す。
        /// 計算量は $O(\log K + \min(m, K))$ ($m$ は走査したユニークな要素数、$K$ は種類数)。
        pub fn nth_max(&self, n: usize) -> Option<&T>
        where
            T: Ord,
        {
            let mut sum = 0;
            for (val, &cnt) in self.iter().rev() {
                if sum + cnt > n {
                    return Some(val);
                }
                sum += cnt;
            }
            None
        }
        /// 指定した範囲内での最小の要素を返す。
        /// 範囲内に要素がない場合は `None` を返す。計算量は $O(\log K)$ ($K$ は種類数)。
        pub fn min_in_range<R>(&self, range: R) -> Option<&T>
        where
            T: Ord,
            R: RangeBounds<T>,
        {
            self.range(range).next().map(|(k, _)| k)
        }
        /// 指定した範囲内での最大の要素を返す。
        /// 範囲内に要素がない場合は `None` を返す。計算量は $O(\log K)$ ($K$ は種類数)。
        pub fn max_in_range<R>(&self, range: R) -> Option<&T>
        where
            T: Ord,
            R: RangeBounds<T>,
        {
            self.range(range).next_back().map(|(k, _)| k)
        }
        /// 指定した範囲内で、重複を考慮して $n$ 番目に小さい要素を返す（0-indexed）。
        /// $n$ が範囲内の要素数以上の場合は `None` を返す。
        /// 計算量は $O(\log K + \min(m, K))$ ($m$ は範囲内で走査したユニークな要素数、$K$ は種類数)。
        pub fn nth_min_in_range<R>(&self, n: usize, range: R) -> Option<&T>
        where
            T: Ord,
            R: RangeBounds<T>,
        {
            let mut sum = 0;
            for (val, &cnt) in self.range(range) {
                if sum + cnt > n {
                    return Some(val);
                }
                sum += cnt;
            }
            None
        }
        /// 指定した範囲内で、重複を考慮して $n$ 番目に大きい要素を返す（0-indexed）。
        /// $n$ が範囲内の要素数以上の場合は `None` を返す。
        /// 計算量は $O(\log K + \min(m, K))$ ($m$ は範囲内で走査したユニークな要素数、$K$ は種類数)。
        pub fn nth_max_in_range<R>(&self, n: usize, range: R) -> Option<&T>
        where
            T: Ord,
            R: RangeBounds<T>,
        {
            let mut sum = 0;
            for (val, &cnt) in self.range(range).rev() {
                if sum + cnt > n {
                    return Some(val);
                }
                sum += cnt;
            }
            None
        }
        pub fn insert(&mut self, value: T)
        where
            T: Ord,
        {
            *self.map.entry(value).or_insert(0) += 1;
            self.length += 1;
        }
        pub fn remove1<Q>(&mut self, value: &Q) -> bool
        where
            T: Borrow<Q> + Ord,
            Q: ?Sized + Ord,
        {
            if let Some(cnt) = self.map.get_mut(value) {
                *cnt -= 1;
                if *cnt == 0 {
                    self.map.remove(value);
                }
                self.length -= 1;
                return true;
            }
            false
        }
        pub fn remove_all<Q>(&mut self, value: &Q) -> bool
        where
            T: Borrow<Q> + Ord,
            Q: ?Sized + Ord,
        {
            if let Some(cnt) = self.map.get(value) {
                self.length -= cnt;
                self.map.remove(value);
                return true;
            }
            false
        }
        pub fn len(&self) -> usize {
            self.length
        }
        pub fn set_len(&self) -> usize {
            self.map.len()
        }
        pub fn is_empty(&self) -> bool {
            self.length == 0
        }
        pub fn count<Q>(&self, value: &Q) -> usize
        where
            T: Borrow<Q> + Ord,
            Q: ?Sized + Ord,
        {
            self.map.get(value).copied().unwrap_or(0)
        }
        pub fn contains<Q>(&self, value: &Q) -> bool
        where
            T: Borrow<Q> + Ord,
            Q: ?Sized + Ord,
        {
            self.map.contains_key(value)
        }
    }
    impl<T: Ord> FromIterator<T> for BTreeMultiSet<T> {
        fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> BTreeMultiSet<T> {
            let mut set = BTreeMultiSet::new();
            for x in iter {
                set.insert(x);
            }
            set
        }
    }
}
