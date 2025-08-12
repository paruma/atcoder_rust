//#[derive_readable]
#[derive(Debug, Clone)]
struct Problem {
    n: usize,
    xs: Vec<usize>,
}

/// all unique な最大連続部分列長を求める
fn solve_sub2(xs: &[usize]) -> usize {
    // 尺取法を使わない実装
    if xs.is_empty() {
        return 0;
    }
    let mut prev_map: HashMap<usize, usize> = HashMap::new();

    let mut begin = 0;
    let mut max_cnt = 0;

    for i in 0..xs.len() {
        if let Some(prev) = prev_map.get(&xs[i]) {
            max_cnt = max_cnt.max(i - begin); // 区間 [begin, i) の長さで更新する
            begin = begin.max(prev + 1);
        }
        prev_map.insert(xs[i], i);
    }
    max_cnt = max_cnt.max(xs.len() - begin); // 区間 [begin, xs.len()) の長さで更新する

    max_cnt
}

/// all unique な最大連続部分列長を求める
fn solve_sub22(xs: &[usize]) -> usize {
    // 尺取法 (begin を 1ずつ動かす)
    if xs.is_empty() {
        return 0;
    }

    let mut begin = 0;
    let mut end = 0;
    let mut set = HashSet::new(); // xs[begin..end] を集合にしたもの
    let mut max_len = 0;
    while begin < xs.len() {
        // xs[begin..end] が all unique であるように end を動かす
        while end < xs.len() {
            if set.contains(&xs[end]) {
                break;
            }

            set.insert(&xs[end]);
            end += 1;
        }

        max_len = max_len.max(end - begin);

        if begin == end {
            begin += 1;
            end += 1;
        } else {
            set.remove(&xs[begin]);
            begin += 1;
        }
    }
    max_len
}

/// all unique な最大連続部分列長を求める
fn solve_sub23(xs: &[usize]) -> usize {
    // 尺取法 (end を1 ずつ動かす)
    if xs.is_empty() {
        return 0;
    }

    let mut begin = 0;
    let mut bag = HashBag::new();
    let mut max_len = 0;

    for end in 1..=xs.len() {
        bag.insert(xs[end - 1]);

        // set_len() != len() は重複があるということ
        while bag.set_len() != bag.len() {
            bag.remove(&xs[begin]);
            begin += 1;
        }

        max_len = max_len.max(end - begin)
    }

    max_len
}

/// None を含まない all unique な最大連続部分列長を求める
fn solve_sub1_old(xs: &[Option<usize>]) -> usize {
    // None で分けて計算をする
    let mut buf = vec![];
    let mut max = 0;

    for &x in xs {
        if let Some(x) = x {
            buf.push(x);
        } else {
            max = max.max(solve_sub2(&buf));
            buf.clear();
        }
    }

    max = max.max(solve_sub2(&buf));
    max
}

/// None を含まない all unique な最大連続部分列長を求める
fn solve_sub1(xs: &[Option<usize>]) -> usize {
    xs.split(|x| x.is_none())
        .map(|chunk| {
            let chunk = chunk.iter().copied().map(|x| x.unwrap()).collect_vec();
            solve_sub23(&chunk)
        })
        .max()
        .unwrap()
}

/// 1122列か判定する
fn solve_naive_sub(s: &[usize]) -> bool {
    let len = s.len();
    if len % 2 == 1 {
        return false;
    }

    if !(1..=len / 2).all(|i| s[2 * i - 2] == s[2 * i - 1]) {
        return false;
    }

    s.iter()
        .copied()
        .counts()
        .values()
        .copied()
        .all(|cnt| cnt == 0 || cnt == 2)
}

impl Problem {
    fn read() -> Problem {
        input! {
            n: usize,
            xs: [usize; n],
        }
        Problem { n, xs }
    }

    fn solve(&self) -> Answer {
        let n = self.n;
        let xs = &self.xs;
        let ys = &xs[1..];

        // chunks_exact の代わりに tuples を使っても良い
        // xs の添字でいうと次のように区切る: [0, 1], [2, 3],...
        let xs1 = xs
            .chunks_exact(2)
            .map(|chunk| (chunk[0] == chunk[1]).then_some(chunk[0]))
            .collect_vec();

        // xs の添字でいうと次のように区切る: [1, 2], [3, 4],...
        let ys1 = ys
            .chunks_exact(2)
            .map(|chunk| (chunk[0] == chunk[1]).then_some(chunk[0]))
            .collect_vec();

        // let xs1 = (0..xs.len() / 2)
        //     .map(|i| (xs[2 * i] == xs[2 * i + 1]).then_some(xs[2 * i]))
        //     .collect_vec();
        // let ys1 = (0..ys.len() / 2)
        //     .map(|i| (ys[2 * i] == ys[2 * i + 1]).then_some(ys[2 * i]))
        //     .collect_vec();

        let ans1 = solve_sub1(&xs1);
        let ans2 = solve_sub1(&ys1);

        let ans = (ans1.max(ans2) * 2) as i64;
        Answer { ans }
    }

    #[allow(dead_code)]
    fn solve_naive(&self) -> Answer {
        let mut max = 0;
        let n = self.n;
        let xs = &self.xs;

        for begin in 0..n {
            for end in begin..=n {
                let ys = &xs[begin..end];
                if solve_naive_sub(ys) {
                    max = max.max(ys.len());
                }
            }
        }
        let ans = max as i64;
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
        let n = rng.gen_range(1..=10);
        let xs = (0..n).map(|_| rng.gen_range(0..n)).collect_vec();

        let p = Problem { n, xs };
        println!("{:?}", &p);
        p
    }

    #[allow(unreachable_code)]
    #[test]
    fn test_with_naive() {
        let num_tests = 1000000;
        let max_wrong_case = 10; // この件数間違いが見つかったら打ち切り
        let mut rng = SmallRng::seed_from_u64(46);
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

use hashbag::HashBag;
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
