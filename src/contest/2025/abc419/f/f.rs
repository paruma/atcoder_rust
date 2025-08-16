fn main() {
    input! {
        n: usize,
        l: usize,
        ss: [Chars; n],
    }
    use ac_library::ModInt998244353 as Mint;

    let ans = BitSet::all_subset(n)
        .map(|set| {
            let len = set.len();
            let sig = Mint::new(if len % 2 == 0 { 1 } else { -1 });
            let mut dp = vec![HashMap::<Vec<char>, Mint>::new(); l + 1];

            dp[0].insert(vec![], Mint::new(1));

            let ss_sub = set
                .to_iter(n)
                .map(|i| ss[i].clone())
                .collect::<HashSet<_>>();

            let prefixes = ss_sub
                .iter()
                .flat_map(|s| (1..s.len()).map(|end| s[..end].to_vec()).collect_vec())
                .collect::<HashSet<_>>();

            // 各 i in set に対して、ss[i] を含まない

            for i in 0..l {
                // 配る: dp[i] から dp[i+1]
                let mut next_dp: HashMap<Vec<char>, Mint> = HashMap::new();
                for (k, v) in &dp[i] {
                    for c in 'a'..='z' {
                        let mut next = k.clone();
                        next.push(c);

                        if ss_sub.contains(&next) {
                            continue;
                        }
                        for begin in 0..next.len() {
                            let next = &next[begin..];
                            if prefixes.contains(next) {
                                let prev_val = dp[i][k];
                                *next_dp.entry(next.to_vec()).or_default() += prev_val + 1;
                            }
                        }
                        let prev_val = dp[i][k];
                        *next_dp.entry(vec![]).or_default() += prev_val + 1;
                    }
                }
                dp[i + 1] = next_dp;
            }
            dbg!(&dp);
            dp[l].values().sum::<Mint>() * sig
        })
        .sum::<Mint>();
    dbg!(ans.to_rational_str());
    let ans = ans.val();
    println!("{}", ans);
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
        let n = rng.gen_range(1..=10);
        let xs = (0..n).map(|_| rng.gen_range(0..10)).collect_vec();

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
        // let mut rng = SmallRng::from_entropy();
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
use std::sync::Arc;
#[allow(unused_imports)]
use {
    itertools::{chain, iproduct, izip, Itertools},
    proconio::{
        derive_readable, fastout, input,
        marker::{Bytes, Chars, Usize1},
    },
    rand::{rngs::SmallRng, seq::SliceRandom, Rng, SeedableRng},
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

use bitset::*;
#[allow(clippy::module_inception)]
pub mod bitset {
    use itertools::Itertools;
    use std::{
        fmt::{Error, Formatter},
        ops::{BitAnd, BitOr, BitXor, Index, IndexMut},
    };
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub struct BitSet {
        bit: usize,
    }
    impl BitSet {
        #[inline]
        pub fn new(bit: usize) -> BitSet {
            BitSet { bit }
        }
        pub fn to_bit(self) -> usize {
            self.bit
        }
        /// 持っている要素を Vec<usize> で返す
        pub fn to_vec(self, len: usize) -> Vec<usize> {
            (0..len).filter(|i| (self.bit >> i) & 1 == 1).collect_vec()
        }
        /// 持っている要素を Iterator で返す
        pub fn to_iter(self, len: usize) -> impl Iterator<Item = usize> {
            (0..len).filter(move |i| (self.bit >> i) & 1 == 1)
        }
        pub fn contains(self, x: usize) -> bool {
            (self.bit >> x) & 1 == 1
        }
        pub fn len(self) -> usize {
            self.bit.count_ones() as usize
        }
        pub fn inserted(self, x: usize) -> BitSet {
            BitSet::new(self.bit | (1 << x))
        }
        pub fn removed(self, x: usize) -> BitSet {
            BitSet::new(self.bit & !(1 << x))
        }
        pub fn empty() -> BitSet {
            BitSet::new(0)
        }
        pub fn universal_set(size: usize) -> BitSet {
            BitSet::new((1 << size) - 1)
        }
        pub fn complement(self, size: usize) -> BitSet {
            BitSet::new(self.bit ^ ((1 << size) - 1))
        }
        pub fn set_minus(self, other: BitSet) -> BitSet {
            BitSet::new(self.bit & !other.bit)
        }
        pub fn is_empty(self) -> bool {
            self.bit == 0
        }
        pub fn is_subset(self, other: BitSet) -> bool {
            self | other == other
        }
        pub fn all_subset(size: usize) -> impl Iterator<Item = BitSet> {
            (0..(1 << size)).map(BitSet::new)
        }
        pub fn subsets(self) -> impl Iterator<Item = BitSet> {
            std::iter::successors(Some(self.bit), move |x| {
                if *x == 0 {
                    None
                } else {
                    Some((x - 1) & self.bit)
                }
            })
            .map(BitSet::new)
        }
    }
    impl BitAnd for BitSet {
        type Output = BitSet;
        fn bitand(self, rhs: BitSet) -> BitSet {
            BitSet::new(self.bit & rhs.bit)
        }
    }
    impl BitOr for BitSet {
        type Output = BitSet;
        fn bitor(self, rhs: BitSet) -> BitSet {
            BitSet::new(self.bit | rhs.bit)
        }
    }
    impl BitXor for BitSet {
        type Output = BitSet;
        fn bitxor(self, rhs: BitSet) -> BitSet {
            BitSet::new(self.bit ^ rhs.bit)
        }
    }
    use std::fmt::Debug;
    impl Debug for BitSet {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
            f.write_fmt(format_args!("{:#b}", self.bit))?;
            Ok(())
        }
    }
    impl<T> Index<BitSet> for [T] {
        type Output = T;
        fn index(&self, s: BitSet) -> &Self::Output {
            &self[s.to_bit()]
        }
    }
    impl<T> IndexMut<BitSet> for [T] {
        fn index_mut(&mut self, s: BitSet) -> &mut Self::Output {
            &mut self[s.to_bit()]
        }
    }
    impl<T> Index<BitSet> for Vec<T> {
        type Output = T;
        fn index(&self, s: BitSet) -> &Self::Output {
            &self[..][s]
        }
    }
    impl<T> IndexMut<BitSet> for Vec<T> {
        fn index_mut(&mut self, s: BitSet) -> &mut Self::Output {
            &mut self[..][s]
        }
    }
}
use default_hash_map::*;
#[allow(clippy::module_inception)]
pub mod default_hash_map {
    use std::hash::Hash;
    use std::{
        borrow::Borrow,
        collections::{
            hash_map::{Iter, IterMut, Keys, Values, ValuesMut},
            HashMap,
        },
    };
    #[derive(Clone, Debug)]
    pub struct DefaultHashMap<K, V> {
        raw: HashMap<K, V>,
        default: V,
    }
    impl<K, V> DefaultHashMap<K, V> {
        pub fn new(default: V) -> DefaultHashMap<K, V> {
            DefaultHashMap {
                raw: HashMap::new(),
                default,
            }
        }
        pub fn from_hash_map(hash_map: HashMap<K, V>, default: V) -> DefaultHashMap<K, V> {
            DefaultHashMap {
                raw: hash_map,
                default,
            }
        }
        pub fn raw(&mut self) -> &mut HashMap<K, V> {
            &mut self.raw
        }
        pub fn keys(&self) -> Keys<K, V> {
            self.raw.keys()
        }
        pub fn values(&self) -> Values<K, V> {
            self.raw.values()
        }
        pub fn values_mut(&mut self) -> ValuesMut<K, V> {
            self.raw.values_mut()
        }
        pub fn iter(&self) -> Iter<K, V> {
            self.raw.iter()
        }
        pub fn iter_mut(&mut self) -> IterMut<K, V> {
            self.raw.iter_mut()
        }
        pub fn len(&mut self) -> usize {
            self.raw.len()
        }
        pub fn is_empty(&mut self) -> bool {
            self.raw.is_empty()
        }
    }
    impl<K, V> DefaultHashMap<K, V>
    where
        K: Eq + Hash,
    {
        pub fn get<Q>(&self, k: &Q) -> &V
        where
            K: Borrow<Q>,
            Q: ?Sized + Hash + Eq,
        {
            self.raw.get(k).unwrap_or(&self.default)
        }
        pub fn get_mut(&mut self, k: K) -> &mut V
        where
            V: Clone,
        {
            self.raw.entry(k).or_insert(self.default.clone())
        }
        pub fn insert(&mut self, k: K, v: V) -> Option<V> {
            self.raw.insert(k, v)
        }
        pub fn remove<Q>(&mut self, k: &Q) -> Option<V>
        where
            K: Borrow<Q>,
            Q: ?Sized + Hash + Eq,
        {
            self.raw.remove(k)
        }
    }
    impl<K, V> PartialEq for DefaultHashMap<K, V>
    where
        K: Eq + Hash,
        V: PartialEq,
    {
        fn eq(&self, other: &DefaultHashMap<K, V>) -> bool {
            self.raw == other.raw && self.default == other.default
        }
    }
    impl<K, V> Eq for DefaultHashMap<K, V>
    where
        K: Eq + Hash,
        V: Eq,
    {
    }
    impl<K, V> Default for DefaultHashMap<K, V>
    where
        V: Default,
    {
        fn default() -> DefaultHashMap<K, V> {
            DefaultHashMap::new(V::default())
        }
    }
    impl<K, V> std::ops::Index<K> for DefaultHashMap<K, V>
    where
        K: Eq + Hash,
    {
        type Output = V;
        #[inline]
        fn index(&self, key: K) -> &V {
            self.get(&key)
        }
    }
    impl<K, V> std::ops::IndexMut<K> for DefaultHashMap<K, V>
    where
        K: Eq + Hash,
        V: Clone,
    {
        #[inline]
        fn index_mut(&mut self, key: K) -> &mut V {
            self.get_mut(key)
        }
    }
}
use modint_to_rational::*;
pub mod modint_to_rational {
    use ac_library::modint::ModIntBase;
    use num_rational::Rational64;
    pub trait ToRational {
        fn to_rational(&self) -> Option<Rational64>;
        fn to_rational_str(&self) -> String {
            self.to_rational()
                .map(|x| x.to_string())
                .unwrap_or("cannot reconstruct".to_string())
        }
    }
    impl<M: ModIntBase> ToRational for M {
        /// 注意: 1000 * 2000 = 2*10^6 の計算をしている
        fn to_rational(&self) -> Option<Rational64> {
            if self.val() == 0 {
                return Some(Rational64::new(0, 1));
            }
            for denom in 1..1000 {
                let denom_inv = Self::new(denom).inv();
                for numer in -1000..1000 {
                    if *self == denom_inv * Self::new(numer) {
                        return Some(Rational64::new(numer, denom));
                    }
                }
            }
            None
        }
    }
}
