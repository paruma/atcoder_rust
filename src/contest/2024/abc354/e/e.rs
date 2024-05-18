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
    bit: u32,
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
        // 表が同じ/裏が同じでgroup by

        struct Rec {
            n: usize,
            cards: Vec<Card>,
        }

        impl Rec {
            fn new(n: usize, cards: Vec<Card>) -> Self {
                Rec { n, cards }
            }

            fn rec(&self, state: &State, tern: usize, dp: &mut Vec<usize>) -> usize {
                let player = tern % 2;

                if dp[state.bit as usize] != 100 {
                    return dp[state.bit as usize];
                }

                if state.has_finished(&self.cards) {
                    let ans = 1 - player;
                    dp[state.bit as usize] = ans;
                    return ans;
                }

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
                    .any(|winner| player == winner);

                let ans = if can_win { player } else { 1 - player };
                dp[state.bit as usize] = ans;
                ans
            }
        }

        let mut dp = vec![100; 1 << self.n];
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
