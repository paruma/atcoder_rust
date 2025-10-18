#[derive_readable]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Card {
    front: i64,
    back: i64,
}
#[derive(Debug, Clone)]
struct Problem {
    n: usize,
    cards: Vec<Card>,
}

struct State {
    bit: usize,
}

impl State {
    fn new(n: usize) -> Self {
        let bit = (1 << n) - 1;
        State { bit }
    }

    fn remove2(&self, i: usize, j: usize) -> State {
        let next_bit = self.bit & (!(1 << i)) & (!(1 << j));
        State { bit: next_bit }
    }

    fn has_finished(&self, cards: &[Card]) -> bool {
        let remain_cards = (0..cards.len())
            .filter(|i| {
                (self.bit >> i & 1) == 1 // まだ残っている
            })
            .map(|i| cards[i])
            .collect_vec();

        let dup_front = remain_cards.iter().duplicates_by(|card| card.front).count() >= 1;
        let dup_back = remain_cards.iter().duplicates_by(|card| card.back).count() >= 1;
        !(dup_front || dup_back)
    }
}

impl Problem {
    fn read() -> Problem {
        input! {
            n: usize,
            cards: [Card; n],
        }
        Problem { n, cards }
    }
    fn solve(&self) -> Answer {
        // メモ化再帰による解法（コンテスト時の解法）
        struct Rec {
            n: usize,
            cards: Vec<Card>,
        }

        impl Rec {
            fn new(n: usize, cards: Vec<Card>) -> Self {
                Rec { n, cards }
            }

            fn rec(&self, state: &State, tern: usize, dp: &mut Vec<Option<usize>>) -> usize {
                if let Some(ans) = dp[state.bit] {
                    return ans;
                }

                let player = tern % 2;

                let ans = if state.has_finished(&self.cards) {
                    1 - player
                } else {
                    let remain_card_i = (0..self.n)
                        .filter(|i| {
                            ((state.bit >> i) & 1) == 1 // まだ残っている
                        })
                        .collect_vec();

                    let can_win = remain_card_i
                        .iter()
                        .copied()
                        .tuple_combinations()
                        .filter(|(i, j)| {
                            let card1 = self.cards[*i];
                            let card2 = self.cards[*j];
                            card1.front == card2.front || card1.back == card2.back
                        })
                        .map(|(i, j)| {
                            // i と j を state から除く
                            let next_state = state.remove2(i, j);
                            self.rec(&next_state, tern + 1, dp)
                        })
                        .any(|winner| player == winner); // has_finished の処理は ここの any で十分かも

                    if can_win {
                        player
                    } else {
                        1 - player
                    }
                };
                dp[state.bit] = Some(ans);
                ans
            }
        }

        let mut dp = vec![None; 1 << self.n];
        let rec = Rec::new(self.n, self.cards.clone());
        let ans = rec.rec(&State::new(self.n), 0, &mut dp);

        let ans = match ans {
            0 => "Takahashi",
            1 => "Aoki",
            _ => panic!(),
        };
        let ans = ans.to_string();
        Answer { ans }
    }

    fn solve2(&self) -> Answer {
        // メモ化再帰による解法（solve のリファクタリング）
        struct Rec {
            n: usize,
            cards: Vec<Card>,
        }

        impl Rec {
            fn new(n: usize, cards: Vec<Card>) -> Self {
                Rec { n, cards }
            }

            fn rec(&self, cards: BitSet, tern: usize, dp: &mut Vec<Option<usize>>) -> usize {
                if let Some(ans) = dp[cards.to_bit()] {
                    return ans;
                }

                let player = tern % 2;

                // できる操作ができない場合は負け (any の部分で false が返る)
                let can_win = cards
                    .to_vec(self.n)
                    .iter()
                    .copied()
                    .tuple_combinations()
                    .filter(|(i, j)| {
                        let card1 = self.cards[*i];
                        let card2 = self.cards[*j];
                        card1.front == card2.front || card1.back == card2.back
                    })
                    .map(|(i, j)| {
                        let next_cards = cards.remove(i).remove(j);
                        self.rec(next_cards, tern + 1, dp)
                    })
                    .any(|winner| player == winner);

                let ans = if can_win { player } else { 1 - player };
                dp[cards.to_bit()] = Some(ans);
                ans
            }
        }

        let mut dp = vec![None; 1 << self.n];
        let rec = Rec::new(self.n, self.cards.clone());
        let ans = rec.rec(BitSet::universal_set(self.n), 0, &mut dp);

        let ans = match ans {
            0 => "Takahashi",
            1 => "Aoki",
            _ => panic!(),
        };
        let ans = ans.to_string();
        Answer { ans }
    }

    fn solve3(&self) -> Answer {
        // for ループによるDPの解法
        let mut dp = vec![10; 1 << self.n];

        for bit in 0_usize..(1 << self.n) {
            // 1ターンで2ずつ除かれる。
            // self.n - bit.count_ones() が 奇数の場合は、ゲームでは到達不可能なので計算する意味はない。
            // メモ化再帰による解法では、ターン数は自然に計算できるが、
            // forループによる解法の場合はターン数をちょっと工夫して計算する必要がある。

            let tern = (self.n - bit.count_ones() as usize) / 2;
            let player = tern % 2;

            let can_win = (0..self.n)
                .filter(|&i| (bit >> i) & 1 == 1)
                .tuple_combinations()
                .filter(|&(i, j)| {
                    let card1 = self.cards[i];
                    let card2 = self.cards[j];

                    card1.front == card2.front || card1.back == card2.back
                })
                .map(|(i, j)| {
                    let next_bit = bit & !(1 << i) & !(1 << j);
                    dp[next_bit]
                })
                .any(|winner| winner == player);
            dp[bit] = if can_win { player } else { 1 - player };
        }
        let ans = dp[(1 << self.n) - 1];
        // for bit in 0..1 << self.n {
        //     let bit_msg = format!("{:b}", bit);
        //     let msg = format!("dp[{}]={}", bit_msg, dp[bit]);
        //     dbg!(msg);
        // }
        let ans = match ans {
            0 => "Takahashi",
            1 => "Aoki",
            _ => panic!(),
        };
        let ans = ans.to_string();
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
    ans: String,
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
use bitset::*;
#[allow(clippy::module_inception)]
pub mod bitset {
    use itertools::Itertools;
    use std::{
        fmt::{Error, Formatter},
        ops::{BitAnd, BitOr, BitXor},
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
        pub fn contains(self, x: usize) -> bool {
            (self.bit >> x) & 1 == 1
        }
        pub fn count(self) -> usize {
            self.bit.count_ones() as usize
        }
        pub fn insert(self, x: usize) -> BitSet {
            BitSet::new(self.bit | (1 << x))
        }
        pub fn remove(self, x: usize) -> BitSet {
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
        pub fn all_subset(self, size: usize) -> impl Iterator<Item = BitSet> {
            (0..(1 << size)).map(BitSet::new)
        }
        pub fn subset_of(self) -> impl Iterator<Item = BitSet> {
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
}
