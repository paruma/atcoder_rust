#[derive_readable]
struct KyotenInfo {
    n_employees: i64,
    offset: i64,
}
struct Problem {
    n_kyoten: usize, //拠点
    info: Vec<KyotenInfo>,
}

impl Problem {
    fn read() -> Problem {
        input! {
            n_kyoten: usize,
            info: [KyotenInfo; n_kyoten],
        }
        Problem { n_kyoten, info }
    }
    // 参加者カウント
    // 世界標準時でt時~t+1時に参加できるか
    fn count(&self, t: i64) -> i64 {
        self.info
            .iter()
            .filter(|kyoten| {
                // 参加可能な拠点でフィルター
                // 世界標準時でｔ時のとき、t + kyoten.offset時になっている
                let local_time = (t + kyoten.offset) % 24; // ローカルでの開始時刻
                9 <= local_time && local_time <= 17
            })
            .map(|kyoten| kyoten.n_employees)
            .sum()
    }
    fn solve(&self) -> Answer {
        let ans = (0..24).map(|t| self.count(t)).max().unwrap();
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
