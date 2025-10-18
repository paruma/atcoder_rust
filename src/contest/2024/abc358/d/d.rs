//#[derive_readable]
#[derive(Debug, Clone)]
struct Problem {
    n_box: usize,
    n_people: usize,         // 箱を渡す人数 = 購入する箱の数
    box_cnt_list: Vec<i64>,  // 箱の値段 = 箱に入っているお菓子の数
    required_list: Vec<i64>, // それぞれの人に渡したいお菓子の数の下限
}

impl Problem {
    fn read() -> Problem {
        input! {
            n_box: usize,
            n_people: usize,
            box_price_list: [i64; n_box],
            required_list: [i64; n_people],
        }
        Problem {
            n_box,
            n_people,
            box_cnt_list: box_price_list,
            required_list,
        }
    }
    fn solve(&self) -> Answer {
        // 解法: ソートする
        let n_box = self.n_box;
        let n_people = self.n_people;
        let box_price_list = &self.box_cnt_list.iter().copied().sorted().collect_vec();
        let required_list = &self.required_list.iter().copied().sorted().collect_vec();

        let mut required_iter = required_list.iter().copied().peekable();

        let mut sum = 0;

        for &box_price in box_price_list {
            if let Some(&required) = required_iter.peek() {
                if box_price >= required {
                    sum += box_price;
                    required_iter.next();
                }
            }
        }

        let ans = if required_iter.peek().is_some() {
            None
        } else {
            Some(sum)
        };
        Answer { ans }
    }

    fn solve2(&self) -> Answer {
        let box_price_list = &self.box_cnt_list;
        let required_list = &self.required_list;

        let mut box_price_set = box_price_list.iter().copied().collect::<BTreeMultiSet<_>>();

        let mut sum = 0;

        for &required in required_list {
            // range(required..) は lower_bound に相当する
            let box_price_opt = box_price_set.range(required..).next();
            if let Some((&box_price, _)) = box_price_opt {
                sum += box_price;
                box_price_set.remove1(&box_price);
            } else {
                return Answer { ans: None };
            }
        }

        Answer { ans: Some(sum) }
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
    ans: Option<i64>,
}

impl Answer {
    fn print(&self) {
        if let Some(ans) = self.ans {
            println!("{}", ans);
        } else {
            println!("-1");
        }
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
        // let n = rng.random_range(1..=10);
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
        // let mut rng = SmallRng::from_os_rng();
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
    marker::{Bytes, Usize1},
};
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
