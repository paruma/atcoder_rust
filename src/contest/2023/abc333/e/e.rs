//#[derive_readable]
enum Event {
    Find { portion_type: usize },
    Encounter { monster_type: usize },
}
struct Problem {
    n_event: usize,
    events: Vec<Event>,
}

impl Problem {
    fn read() -> Problem {
        input! {
            n_event: usize,
            events: [(usize, Usize1); n_event],
        }

        let events = events
            .iter()
            .copied()
            .map(|(t, x)| match t {
                1 => Event::Find { portion_type: x },
                2 => Event::Encounter { monster_type: x },
                _ => panic!(),
            })
            .collect_vec();
        Problem { n_event, events }
    }
    fn solve(&self) -> Answer {
        let n_event = self.n_event;
        let events = &self.events;
        let mut cnts = vec![0; n_event];
        let mut movement_list_rev = vec![];
        let mut score = 0; // ポーションの最大所持
        let mut max_score = 0; // ポーションの最大所持

        for event in events.iter().rev() {
            match event {
                Event::Find { portion_type } => {
                    if cnts[*portion_type] > 0 {
                        // 拾う
                        cnts[*portion_type] -= 1;
                        score -= 1;
                        movement_list_rev.push(1);
                    } else {
                        // 拾わない
                        movement_list_rev.push(0);
                    }
                }
                Event::Encounter { monster_type } => {
                    cnts[*monster_type] += 1;
                    score += 1;
                    max_score = max_score.max(score);
                }
            }
        }

        let ans = if cnts.iter().any(|&cnt| cnt > 0) {
            None
        } else {
            Some(Result {
                score: max_score,
                movement_list: movement_list_rev.into_iter().rev().collect_vec(),
            })
        };
        Answer { ans }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Result {
    score: i64,              // ポーション持ってた数の最大値
    movement_list: Vec<i64>, // 拾ったかどうか(拾ったら1)
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Answer {
    ans: Option<Result>,
}

impl Answer {
    fn print(&self) {
        if let Some(result) = &self.ans {
            println!("{}", result.score);
            print_vec_1line(&result.movement_list);
        } else {
            println!("-1");
        }
    }
}

fn main() {
    Problem::read().solve().print();
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_problem() {
        assert_eq!(1 + 1, 2);
    }
}

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
