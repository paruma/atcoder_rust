#[derive_readable]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Segment {
    l: Usize1,
    r: Usize1,
}
#[derive(Debug, Clone)]
struct Problem {
    n: usize,
    m: usize,
    segs: Vec<Segment>,
}

impl Problem {
    fn read() -> Problem {
        input! {
            n: usize,
            m: usize,
            segs: [Segment; n]
        }
        Problem { n, m, segs }
    }

    fn solve(&self) -> Answer {
        // l を固定して、条件を満たす r の数を数える。
        // l は昇順に考えた (l を降順または r を昇順に考えるほうが楽)
        let n = self.n;
        let m = self.m;
        let segs = &self.segs;
        let mut segs = segs
            .iter()
            .copied()
            .map(|s| (s.l, s.r))
            .collect::<BTreeSet<(usize, usize)>>();
        let mut rights = segs
            .iter()
            .copied()
            .map(|(left, right)| right)
            .collect::<BTreeMultiSet<usize>>();

        rights.insert(m); // 番兵

        let mut ans = 0;

        for l in 0..m {
            // segs にある左が l 以下の seg を取り消す
            let (seg_r, _cnt) = rights.iter().min().unwrap();

            let ans_sub = seg_r - l;
            ans += ans_sub;

            while let Some(&(seg_l, seg_r)) = segs.iter().min() {
                if seg_l <= l {
                    segs.remove(&(seg_l, seg_r));
                    rights.remove1(&seg_r);
                } else {
                    break;
                }
            }
        }
        Answer { ans }
    }

    fn solve2(&self) -> Answer {
        // r を固定して、条件を満たす l の数を数える。
        // r は昇順に考えた
        let m = self.m;

        let segs_by_r = self
            .segs
            .iter()
            .copied()
            .fold(vec![vec![]; self.m], |mut acc, seg| {
                acc[seg.r].push(seg);
                acc
            });

        let mut min_l = 0; // 各 r に対してどこまで l を左に伸ばせるか
        let mut ans = 0;

        for r in 0..m {
            for seg in &segs_by_r[r] {
                min_l = min_l.max(seg.l + 1);
            }
            ans += r + 1 - min_l;
        }

        Answer { ans }
    }

    #[allow(dead_code)]
    fn solve_naive(&self) -> Answer {
        todo!();
        // let ans = 0;
        // Answer { ans }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Answer {
    ans: usize,
}

impl Answer {
    fn print(&self) {
        println!("{}", self.ans);
    }
}

fn main() {
    Problem::read().solve2().print();
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

    #[allow(dead_code)]
    #[derive(Debug)]
    struct WrongTestCase {
        problem: Problem,
        main_ans: Answer,
        naive_ans: Answer,
    }

    #[allow(dead_code)]
    fn check(p: &Problem) -> Option<WrongTestCase> {
        let main_ans = p.solve();
        let naive_ans = p.solve_naive();
        if main_ans != naive_ans {
            Some(WrongTestCase {
                problem: p.clone(),
                main_ans,
                naive_ans,
            })
        } else {
            None
        }
    }

    #[allow(dead_code)]
    fn make_random_problem(rng: &mut SmallRng) -> Problem {
        todo!()
        // let n = rng.gen_range(1..=10);
        // let p = Problem { _a: n };
        // println!("{:?}", &p);
        // p
    }

    #[allow(unreachable_code)]
    #[test]
    fn test_with_naive() {
        let num_tests = 0;
        let max_wrong_case = 10; // この件数間違いが見つかったら打ち切り
        let mut rng = SmallRng::seed_from_u64(42);
        // let mut rng = SmallRng::from_entropy();
        let mut wrong_cases: Vec<WrongTestCase> = vec![];
        for _ in 0..num_tests {
            let p = make_random_problem(&mut rng);
            let result = check(&p);
            if let Some(wrong_test_case) = result {
                wrong_cases.push(wrong_test_case);
            }
            if wrong_cases.len() >= max_wrong_case {
                break;
            }
        }

        if !wrong_cases.is_empty() {
            for t in &wrong_cases {
                println!("{:?}", t.problem);
                println!("main ans : {:?}", t.main_ans);
                println!("naive ans: {:?}", t.naive_ans);
                println!();
            }
            println!("{} cases are wrong.", wrong_cases.len());
            panic!();
        }
    }
}

// ====== import ======
#[allow(unused_imports)]
use itertools::{chain, iproduct, izip, Itertools};
#[allow(unused_imports)]
use proconio::{
    derive_readable, fastout, input,
    marker::{Bytes, Chars, Usize1},
};
#[allow(unused_imports)]
use std::cmp::Reverse;
use std::collections::BTreeSet;
#[allow(unused_imports)]
use std::collections::{BinaryHeap, HashMap, HashSet};

// ====== output func ======
#[allow(unused_imports)]
use print_vec::*;
pub mod print_vec {

    use itertools::Itertools;
    use proconio::fastout;
    #[fastout]
    pub fn print_vec<T: std::fmt::Debug>(arr: &[T]) {
        for a in arr {
            println!("{:?}", a);
        }
    }
    #[fastout]
    pub fn print_vec_1line<T: std::fmt::Debug>(arr: &[T]) {
        let msg = arr.iter().map(|x| format!("{:?}", x)).join(" ");
        println!("{}", msg);
    }
    #[fastout]
    pub fn print_vec2<T: std::fmt::Debug>(arr: &Vec<Vec<T>>) {
        for row in arr {
            let msg = row.iter().map(|x| format!("{:?}", x)).join(" ");
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
}

#[allow(unused)]
fn print_yesno(ans: bool) {
    let msg = if ans { "Yes" } else { "No" };
    println!("{}", msg);
}

// ====== snippet ======
use btree_multiset::*;
#[allow(clippy::module_inception)]
pub mod btree_multiset {
    use std::{
        borrow::Borrow,
        collections::{btree_map::Range, BTreeMap},
        ops::RangeBounds,
    };
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct BTreeMultiSet<T> {
        map: BTreeMap<T, usize>,
        length: usize,
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
        pub fn iter(&self) -> impl Iterator<Item = (&T, &usize)> {
            self.map.iter()
        }
        pub fn insert(&mut self, value: T)
        where
            T: Ord,
        {
            *self.map.entry(value).or_insert(0) += 1;
            self.length += 1;
        }
        pub fn remove1<Q: ?Sized>(&mut self, value: &Q) -> bool
        where
            T: Borrow<Q> + Ord,
            Q: Ord,
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
        pub fn remove_all<Q: ?Sized>(&mut self, value: &Q) -> bool
        where
            T: Borrow<Q> + Ord,
            Q: Ord,
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
        pub fn is_empty(&self) -> bool {
            self.length == 0
        }
        pub fn count<Q: ?Sized>(&self, value: &Q) -> usize
        where
            T: Borrow<Q> + Ord,
            Q: Ord,
        {
            self.map.get(value).copied().unwrap_or(0)
        }
        pub fn contains<Q: ?Sized>(&self, value: &Q) -> bool
        where
            T: Borrow<Q> + Ord,
            Q: Ord,
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
