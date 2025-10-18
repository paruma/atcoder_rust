//#[derive_readable]

#[derive(Clone, Debug, PartialEq, Eq)]
struct Problem {
    n: i64,
    s: Vec<u8>,
    t: Vec<u8>,
}

struct SubProblem {
    s: Vec<usize>,
    cnt_cumsum_in_s: Vec<CumSum>,
    idx_list_in_s: Vec<Vec<usize>>,
}

impl SubProblem {
    fn new(s: &[usize]) -> SubProblem {
        let s = s.to_vec();
        let cnt_cumsum_in_s = (0..26)
            .map(|c| {
                let ind = s.iter().copied().map(|x| (x == c) as i64).collect_vec();
                CumSum::new(&ind)
            })
            .collect_vec();

        let idx_list_in_s =
            s.iter()
                .copied()
                .enumerate()
                .fold(vec![vec![]; 26], |mut acc, (i, c)| {
                    acc[c].push(i);
                    acc
                });
        SubProblem {
            s,
            cnt_cumsum_in_s,
            idx_list_in_s,
        }
    }
    // g(t, k) を部分列として持つために必要な s の繰り返し回数
    // g('abc', 3) = 'aaabbbccc'
    fn solve_sub(&self, t: &[usize], k: i64) -> i64 {
        let s = &self.s;
        let cnt_cumsum_in_s = &self.cnt_cumsum_in_s;
        let idx_list_in_s = &self.idx_list_in_s;
        let mut x = 0;
        let mut y = 0;

        for &c in t {
            //let ch = (b'a' + c as u8) as char;
            // c が少ない場合
            // 今のレイヤーでの残りの数
            let cnt_remain_current_y = cnt_cumsum_in_s[c].suffix_sum(x);
            if cnt_remain_current_y >= k {
                // 今のレイヤーでk個進める
                let i = lower_bound(&idx_list_in_s[c], x);
                let next_i = i + k as usize - 1;
                x = idx_list_in_s[c][next_i] + 1;
            } else {
                // 今のレイヤーで cnt_remain_current_y 個進める
                // k - cnt_remain_current_y 個消費したい
                let next_k = k - cnt_remain_current_y;
                let cnt_y = next_k / cnt_cumsum_in_s[c].all_sum();
                let remain_k = next_k % cnt_cumsum_in_s[c].all_sum();
                if remain_k == 0 {
                    y += cnt_y;
                    x = idx_list_in_s[c].last().unwrap() + 1;
                } else {
                    y += cnt_y + 1;
                    x = idx_list_in_s[c][remain_k as usize - 1] + 1;
                }
                // オーバーフロー回避
                if y >= 1e18 as i64 {
                    return y;
                }
            }
        }

        if x == 0 {
            y -= 1;
            x = s.len();
        }

        y + 1
    }
}

impl Problem {
    fn read() -> Problem {
        input! {
            n: i64,
            s: Bytes,
            t: Bytes,
        }
        Problem { n, s, t }
    }

    fn to_dbg_str(&self) -> String {
        format!(
            "{} {} {}",
            self.n,
            String::from_utf8(self.s.clone()).unwrap(),
            String::from_utf8(self.t.clone()).unwrap()
        )
    }

    fn solve(&self) -> Answer {
        let s = self
            .s
            .iter()
            .copied()
            .map(|c| (c - b'a') as usize)
            .collect_vec();
        let t = self
            .t
            .iter()
            .copied()
            .map(|c| (c - b'a') as usize)
            .collect_vec();

        // t の中に sにない文字があったら0
        let is_zero = {
            let s_set = s.iter().copied().collect::<HashSet<usize>>();
            t.iter().any(|x| !s_set.contains(x))
        };
        if is_zero {
            return Answer { ans: 0 };
        }

        let sub = SubProblem::new(&s);

        let n = self.n;

        let ans = bin_search(0, 1e18 as i64, |k| sub.solve_sub(&t, k) <= n);
        Answer { ans }
    }

    fn solve_naive(&self) -> Answer {
        fn is_subseq(xs: &[u8], ys: &[u8]) -> bool {
            // xs が ys の（連続とは限らない）部分列？
            let mut xsi = xs.iter().copied().peekable();

            for &y in ys {
                if Some(y) == xsi.peek().copied() {
                    xsi.next();
                }
            }
            xsi.peek().is_none()
        }
        let fsn = std::iter::repeat(&self.s)
            .take(self.n as usize)
            .flatten()
            .copied()
            .collect_vec();
        let ans = (0..)
            .find(|&k| {
                let gtk = self
                    .t
                    .iter()
                    .copied()
                    .flat_map(|c| std::iter::repeat(c).take(k))
                    .collect_vec();
                !is_subseq(&gtk, &fsn)
            })
            .unwrap() as i64
            - 1;

        Answer { ans }
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
    Problem::read().solve().print();
}

#[cfg(test)]
mod tests {
    use rand::{rngs::SmallRng, Rng, SeedableRng};

    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_problem() {
        let s = &[0, 1, 2];
        let t = &[0, 1];
        let p = SubProblem::new(s);
        assert_eq!(p.solve_sub(t, 3), 5);
        assert_eq!(p.solve_sub(t, 4), 7);
    }

    fn str_to_vec_usize(s: &str) -> Vec<usize> {
        s.bytes().map(|ch| (ch - b'a') as usize).collect_vec()
    }

    #[test]
    fn test_problem2() {
        let s = str_to_vec_usize("kzazkakxkk");
        let t = str_to_vec_usize("azakxk");
        let p = SubProblem::new(&s);
        assert_eq!(p.solve_sub(&t, 2), 4);
        assert_eq!(p.solve_sub(&t, 3), 8);
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
        let n = rng.random_range(1..=10);
        let s_len = rng.random_range(1..=10);
        let t_len = rng.random_range(1..=10);
        let alphabet = b"abc".to_vec();
        let s = (0..s_len)
            .map(|_| alphabet[rng.random_range(0..alphabet.len())])
            .collect_vec();
        let t = (0..t_len)
            .map(|_| alphabet[rng.random_range(0..alphabet.len())])
            .collect_vec();

        let p = Problem { n, s, t };
        println!("{}", p.to_dbg_str());
        p
    }

    #[allow(unreachable_code)]
    #[test]
    fn test_with_naive() {
        let num_tests = 200;
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
                println!("{:?}", t.problem.to_dbg_str());
                println!("main ans : {:?}", t.main_ans);
                println!("naive ans: {:?}", t.naive_ans);
                println!();
            }
            println!("{} cases are wrong.", wrong_cases.len());
            panic!();
        }
    }
}

use std::collections::HashSet;

// ====== import ======
#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use proconio::{
    derive_readable, fastout, input,
    marker::{Bytes, Usize1},
};

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
use cumsum::*;
pub mod cumsum {
    pub fn prefix_sum(xs: &[i64]) -> Vec<i64> {
        let mut prefix_sum = vec![0; xs.len() + 1];
        for i in 1..xs.len() + 1 {
            prefix_sum[i] = prefix_sum[i - 1] + xs[i - 1];
        }
        prefix_sum
    }
    use std::ops::{Bound, Range, RangeBounds};
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct CumSum {
        pub cumsum: Vec<i64>,
    }
    impl CumSum {
        /// 計算量: O(|xs|)
        pub fn new(xs: &[i64]) -> CumSum {
            let mut cumsum = vec![0; xs.len() + 1];
            for i in 1..xs.len() + 1 {
                cumsum[i] = cumsum[i - 1] + xs[i - 1];
            }
            CumSum { cumsum }
        }
        fn open(&self, range: impl RangeBounds<usize>) -> Range<usize> {
            use Bound::Excluded;
            use Bound::Included;
            use Bound::Unbounded;
            let begin = match range.start_bound() {
                Unbounded => 0,
                Included(&x) => x,
                Excluded(&x) => x + 1,
            };
            let end = match range.end_bound() {
                Excluded(&x) => x,
                Included(&x) => x + 1,
                Unbounded => self.cumsum.len() - 1,
            };
            begin..end
        }
        /// 計算量: O(1)
        pub fn range_sum(&self, range: impl RangeBounds<usize>) -> i64 {
            let range = self.open(range);
            self.cumsum[range.end] - self.cumsum[range.start]
        }
        pub fn prefix_sum(&self, end: usize) -> i64 {
            self.cumsum[end]
        }
        pub fn suffix_sum(&self, begin: usize) -> i64 {
            self.cumsum[self.cumsum.len() - 1] - self.cumsum[begin]
        }
        pub fn all_sum(&self) -> i64 {
            self.cumsum[self.cumsum.len() - 1]
        }
    }
}
/// 二分探索をする
/// ```text
/// ng ng ng ok ok ok
///          ↑ここの引数の値を返す
/// ```
/// 計算量: O(log(|ok - ng|))
/// ## Arguments
/// * ok != ng
/// * |ok - ng| <= 2^63 - 1, |ok + ng| <= 2^63 - 1
/// * p の定義域について
///     * ng < ok の場合、p は区間 ng..ok で定義されている。
///     * ok < ng の場合、p は区間 ok..ng で定義されている。
/// * p の単調性について
///     * ng < ok の場合、p は単調増加
///     * ok < ng の場合、p は単調減少
/// ## Return
/// * ng < ok の場合: I = { i in ng..ok | p(i) == true } としたとき
///     * I が空でなければ、min I を返す。
///     * I が空ならば、ok を返す。
/// * ok < ng の場合: I = { i in ok..ng | p(i) == true } としたとき
///     * I が空でなければ、max I を返す。
///     * I が空ならば、ok を返す。
pub fn bin_search<F>(mut ok: i64, mut ng: i64, p: F) -> i64
where
    F: Fn(i64) -> bool,
{
    assert!(ok != ng);
    assert!(ok.checked_sub(ng).is_some());
    assert!(ok.checked_add(ng).is_some());
    while num::abs(ok - ng) > 1 {
        let mid = (ok + ng) / 2;
        assert!(mid != ok);
        assert!(mid != ng);
        if p(mid) {
            ok = mid;
        } else {
            ng = mid;
        }
    }
    ok
}
/// 指定された要素以上の値が現れる最初の位置を返す。
/// 計算量: O(log(|xs|))
/// ## Arguments
/// * xs: 単調増加
///     * 単調増加でなくても、 `|i| xs[i] >= key` が単調ならOK
/// ## Return
/// `I = {i in 0..xs.len() | xs[i] >= key}` としたとき、`min I` を返す。
/// ただし、`I` が空の場合は `xs.len()` を返す
/// 戻り値は、区間 `0..=xs.len()` の間で返る。
pub fn lower_bound<T: PartialOrd>(xs: &[T], key: T) -> usize {
    let pred = |i: i64| xs[i as usize] >= key;
    bin_search(xs.len() as i64, -1_i64, pred) as usize
}
