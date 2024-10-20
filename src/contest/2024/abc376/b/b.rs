#[derive_readable]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Query {
    hand: char,
    target: Usize1,
}

#[derive(Debug, Clone)]
struct Problem {
    n: usize,
    nq: usize,
    qs: Vec<Query>,
}

/// 円環
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Ring {
    len: usize,
}

impl Ring {
    fn new(len: usize) -> Ring {
        Ring { len }
    }
    fn inc(&self, x: usize) -> usize {
        (x + 1) % self.len
    }

    fn dec(&self, x: usize) -> usize {
        (x + self.len - 1) % self.len
    }
    /// 時計回りに src から dst に移動したときの道のり
    fn dist_right(&self, src: usize, dst: usize) -> usize {
        assert!((0..self.len).contains(&src));
        assert!((0..self.len).contains(&dst));
        if src > dst {
            dst + self.len - src
        } else {
            dst - src
        }
    }

    /// 反時計回りに src から dst に移動したときの道のり
    fn dist_left(&self, src: usize, dst: usize) -> usize {
        assert!((0..self.len).contains(&src));
        assert!((0..self.len).contains(&dst));
        self.dist_right(dst, src)
    }

    /// begin から end に時計回りに回ったときに x に当たるかどうか
    fn contains(&self, begin: usize, end: usize, x: usize) -> bool {
        assert!((0..self.len).contains(&begin));
        assert!((0..self.len).contains(&end));
        if end < begin {
            (begin..end + self.len).contains(&x)
                || (begin..end + self.len).contains(&(x + self.len))
        } else {
            (begin..end).contains(&x)
        }
    }
}

impl Problem {
    fn read() -> Problem {
        input! {
            n: usize,
            nq: usize,
            qs: [Query; nq],
        }
        Problem { n, nq, qs }
    }

    fn solve(&self) -> Answer {
        // 愚直シミュレーション
        let n = self.n;
        let qs = &self.qs;

        let mut left = 0;
        let mut right = 1;
        let mut cnt = 0;
        let n = self.n;

        let inc = |x: usize| (x + 1) % n;
        let dec = |x: usize| (x + n - 1) % n;

        for q in &self.qs {
            if q.hand == 'R' {
                let mut c_right = right;
                let mut fail = false;
                let mut c_cnt = 0;
                // 左に動かす
                while c_right != q.target {
                    c_right = dec(c_right);
                    c_cnt += 1;
                    if c_right == left {
                        fail = true;
                        break;
                    }
                }
                if !fail {
                    cnt += c_cnt;
                    right = c_right;
                } else {
                    // 右に動かす
                    let mut c_right = right;
                    let mut c_cnt = 0;
                    // 左に動かす
                    while c_right != q.target {
                        c_right = inc(c_right);
                        c_cnt += 1;
                    }
                    right = c_right;
                    cnt += c_cnt;
                }
            } else if q.hand == 'L' {
                let mut c_left = left;
                let mut fail = false;
                let mut c_cnt = 0;
                // 左に動かす
                while c_left != q.target {
                    c_left = dec(c_left);
                    c_cnt += 1;
                    if c_left == right {
                        fail = true;
                        break;
                    }
                }
                if !fail {
                    cnt += c_cnt;
                    left = c_left;
                } else {
                    // 右に動かす
                    let mut c_left = left;
                    let mut c_cnt = 0;

                    // 左に動かす
                    while c_left != q.target {
                        c_left = inc(c_left);
                        c_cnt += 1;
                    }
                    left = c_left;
                    cnt += c_cnt;
                }
            } else {
                unreachable!();
            }
        }

        let ans = cnt;
        Answer { ans }
    }

    fn solve2(&self) -> Answer {
        // solve のリファクタリング (愚直シミュレーション)
        let n = self.n;
        let inc = |x: usize| (x + 1) % n;
        let dec = |x: usize| (x + n - 1) % n;

        let mut left = 0;
        let mut right = 1;

        let mut ans = 0;

        for q in &self.qs {
            let (moving_hand, other_hand) = if q.hand == 'R' {
                (&mut right, left)
            } else {
                (&mut left, right)
            };

            // 右に動かす
            let moving_right_len = (|| {
                let mut current_hand = *moving_hand;
                let mut cnt = 0;
                while current_hand != q.target {
                    current_hand = inc(current_hand);
                    cnt += 1;
                    if current_hand == other_hand {
                        return None;
                    }
                }
                Some(cnt)
            })();
            // 左に動かす
            let moving_left_len = (|| {
                let mut current_hand = *moving_hand;
                let mut cnt = 0;
                while current_hand != q.target {
                    current_hand = dec(current_hand);
                    cnt += 1;
                    if current_hand == other_hand {
                        return None;
                    }
                }
                Some(cnt)
            })();

            // *moving_had == q.target の場合はどちらも Some(0) になるので、xor にはできない。
            let current_cnt = moving_left_len.or(moving_right_len).unwrap();
            ans += current_cnt;
            *moving_hand = q.target;
        }
        Answer { ans }
    }

    fn solve3(&self) -> Answer {
        // O(Q) で計算
        let n = self.n;

        let mut left = 0;
        let mut right = 1;

        let ring = Ring::new(n);

        let mut ans = 0;

        for q in &self.qs {
            let (moving_hand, other_hand) = if q.hand == 'R' {
                (&mut right, left)
            } else {
                (&mut left, right)
            };
            // moving_hand から q.target に右に動かしたとき other_hand があるか
            let moving_len = if ring.contains(*moving_hand, q.target, other_hand) {
                // 左に動かす
                ring.dist_left(*moving_hand, q.target)
            } else {
                // 右に動かす
                ring.dist_right(*moving_hand, q.target)
            };
            ans += moving_len;
            *moving_hand = q.target;
        }

        let ans = ans as i64;
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
    ans: i64,
}

impl Answer {
    fn print(&self) {
        println!("{}", self.ans);
    }
}

fn main() {
    Problem::read().solve3().print();
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
