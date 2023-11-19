//#[derive_readable]
struct Problem {
    n_cand: usize,
    n_votes: usize,
    votes: Vec<usize>, // votes[i]] = i番目の票の投票先
}

impl Problem {
    fn read() -> Problem {
        input! {
            n_cand: usize,
            n_votes: usize,
            votes: [Usize1; n_votes],// votes[i]] = i番目の票の投票先
        }
        Problem { n_cand, n_votes, votes }
    }
    #[allow(clippy::comparison_chain)]
    #[allow(clippy::collapsible_if)]
    fn solve(&self) -> Answer {
        let Problem { n_cand, n_votes, votes } = self;
        let mut top_cand = 0;
        let mut top_cnt = 0;
        let mut ans: Vec<usize> = vec![];
        let mut cnt = vec![0; *n_cand];
        for (_vote_i, cand_i) in votes.iter().copied().enumerate() {
            cnt[cand_i] += 1;
            if cnt[cand_i] > top_cnt {
                top_cnt = cnt[cand_i];
                top_cand = cand_i;
            } else if cnt[cand_i] == top_cnt {
                if cand_i < top_cand {
                    top_cnt = cnt[cand_i];
                    top_cand = cand_i;
                }
            }
            // dbg!(top_cand);
            // dbg!(top_cnt);
            ans.push(top_cand);
        }

        Answer { ans }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Answer {
    ans: Vec<usize>,
}

impl Answer {
    fn print(&self) {
        print_vec(&self.ans.iter().copied().map(|x| x + 1).collect_vec());
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
