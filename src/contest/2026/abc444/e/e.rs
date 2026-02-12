// r を 1ずつうごかす (min_left)
// 多重集合を BTreeMap で管理

// #[fastout]
fn main() {
    input! {
        n: usize,
        d: i64,
        xs: [i64; n],
    }
    let mut bag = BTreeMultiSet::new();

    let mut l = 0_usize;
    let mut sum = 0;
    for r in 0..n {
        // l を伸ばす (l<=r まで)
        // xs[l..=r] の要素のうち、開区間 (xs[r] - d, xs[r] + d) に含まれている個数 (0であってほしい)
        loop {
            let contains = bag.range(xs[r] - d + 1..xs[r] + d).next().is_some();
            if contains {
                bag.remove1(&xs[l]);
                l += 1;
            } else {
                break;
            }
        }
        let sub = r - l + 1;
        sum += sub;

        bag.insert(xs[r]);
    }

    println!("{}", sum);
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
        pub fn iter(&self) -> impl Iterator<Item = &T> {
            self.map
                .iter()
                .flat_map(|(e, cnt)| std::iter::repeat_n(e, *cnt))
        }
        pub fn set_iter(&self) -> impl Iterator<Item = (&T, usize)> {
            self.map.iter().map(|(e, cnt)| (e, *cnt))
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
