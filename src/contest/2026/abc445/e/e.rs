// 全体から1つだけ消したらどうなるかというのを考えていく
// top2 を管理するとうまくいく

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

        let mut prime_to_exps = HashMap::<usize, Top2Multiset<usize>>::new();

        for f in &xs_f {
            for &(p, exp) in f {
                prime_to_exps.entry(p).or_default().insert(exp);
            }
        }

        let lcm: Mint = prime_to_exps
            .iter()
            .map(|(p, cnts)| Mint::new(*p).pow(cnts.nth(0).unwrap_or(0) as u64))
            .product();

        // dbg!(lcm);
        let mut ans = vec![];

        for f in &xs_f {
            let div = f
                .iter()
                .copied()
                .map(|(p, cnt)| {
                    let exps_top2 = prime_to_exps[&p];
                    // exps_top2 から cnt を除いた場合の差分を計算する
                    if exps_top2.nth(0) == Some(cnt) {
                        let first = exps_top2.nth(0).unwrap_or(0);
                        let second = exps_top2.nth(1).unwrap_or(0);
                        Mint::new(p).pow((first - second) as u64)
                    } else {
                        Mint::new(1)
                    }
                })
                .product::<Mint>();

            ans.push(lcm / div);
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
use topk_multiset::*;
#[allow(clippy::module_inception)]
pub mod topk_multiset {
    use std::fmt;
    /// 値が大きい方から最大 K 個を保持するマルチセット（同一値の重複あり）。
    /// ヒープを使用せず、スタック上の固定長配列で動作する。
    #[derive(Clone, Copy, PartialEq, Eq, Hash)]
    pub struct TopKMultiset<T, const K: usize> {
        buf: [T; K],
        len: usize,
    }
    /// 値が大きい方から最大 2 個を保持するマルチセット。
    pub type Top2Multiset<T> = TopKMultiset<T, 2>;
    /// 値が大きい方から最大 3 個を保持するマルチセット。
    pub type Top3Multiset<T> = TopKMultiset<T, 3>;
    /// 値が大きい方から最大 4 個を保持するマルチセット。
    pub type Top4Multiset<T> = TopKMultiset<T, 4>;
    /// 値が大きい方から最大 5 個を保持するマルチセット。
    pub type Top5Multiset<T> = TopKMultiset<T, 5>;
    impl<T, const K: usize> TopKMultiset<T, K>
    where
        T: Ord + Copy + Default,
    {
        /// 空の TopKMultiset を作成する。
        /// 計算量は $O(1)$。
        pub fn new() -> Self {
            Self {
                buf: [T::default(); K],
                len: 0,
            }
        }
        /// 要素を 1 つだけ含む TopKMultiset を作成する。
        /// 計算量は $O(K)$。
        pub fn unit(value: T) -> Self {
            Self::new().inserted(value)
        }
        /// 要素を 1 つ追加する。
        /// 計算量は $O(K)$。
        pub fn insert(&mut self, value: T) {
            let pos = self.buf[..self.len]
                .iter()
                .position(|&x| value >= x)
                .unwrap_or(self.len);
            if self.len < K {
                self.len += 1;
            } else if pos < K {
            } else {
                return;
            }
            let end = self.len.min(K);
            self.buf.copy_within(pos..end - 1, pos + 1);
            self.buf[pos] = value;
        }
        /// 要素を 1 つ追加した新しい TopKMultiset を返す。
        /// 計算量は $O(K)$。
        #[must_use]
        pub fn inserted(self, value: T) -> Self {
            let mut result = self;
            result.insert(value);
            result
        }
        /// other の全要素を追加した新しい TopKMultiset を返す。
        /// 計算量は $O(K^2)$。
        #[must_use]
        pub fn merged(self, other: Self) -> Self {
            let mut result = self;
            result.merge(other);
            result
        }
        /// other の全要素を追加する。
        /// 計算量は $O(K^2)$。
        pub fn merge(&mut self, other: Self) {
            for x in other.iter() {
                self.insert(x);
            }
        }
        /// i 番目に大きい要素を返す（0-indexed）。
        /// i >= len の場合は None を返す。計算量は $O(1)$。
        pub fn nth(&self, i: usize) -> Option<T> {
            if i < self.len {
                Some(self.buf[i])
            } else {
                None
            }
        }
        /// 保持している最大の要素を返す。
        /// `nth(0)` と同じ。計算量は $O(1)$。
        pub fn max(&self) -> Option<T> {
            self.nth(0)
        }
        /// 保持している要素数を返す。
        /// 計算量は $O(1)$。
        pub fn len(&self) -> usize {
            self.len
        }
        /// 空かどうかを返す。
        /// 計算量は $O(1)$。
        pub fn is_empty(&self) -> bool {
            self.len == 0
        }
        /// 保持している要素のイテレータを返す（T 降順）。
        pub fn iter(&self) -> impl Iterator<Item = T> + '_ {
            self.buf[..self.len].iter().copied()
        }
    }
    impl<T, const K: usize> Default for TopKMultiset<T, K>
    where
        T: Ord + Copy + Default,
    {
        fn default() -> Self {
            Self::new()
        }
    }
    impl<T, const K: usize> FromIterator<T> for TopKMultiset<T, K>
    where
        T: Ord + Copy + Default,
    {
        /// イテレータの各要素を順に insert した結果と等価。
        /// 計算量は $O(NK)$（N はイテレータの要素数）。
        fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
            let mut result = Self::new();
            for x in iter {
                result.insert(x);
            }
            result
        }
    }
    impl<T: Copy, const K: usize> IntoIterator for TopKMultiset<T, K> {
        type Item = T;
        type IntoIter = std::iter::Take<std::array::IntoIter<T, K>>;
        fn into_iter(self) -> Self::IntoIter {
            self.buf.into_iter().take(self.len)
        }
    }
    impl<T: fmt::Debug, const K: usize> fmt::Debug for TopKMultiset<T, K> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{{")?;
            for (i, x) in self.buf[..self.len].iter().enumerate() {
                if i > 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{:?}", x)?;
            }
            write!(f, "}}")
        }
    }
}
