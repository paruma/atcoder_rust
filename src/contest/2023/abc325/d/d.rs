struct Problem {
    items: Vec<Item>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[derive_readable]
struct Item {
    time: usize,
    duration: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Range {
    begin: usize,
    end: usize,
}

impl Problem {
    fn read() -> Problem {
        input! {
            n: usize,
            items: [Item; n],
        }
        Problem { items }
    }
    fn solve(&self) -> Answer {
        let ranges = self
            .items
            .iter()
            .copied()
            .map(|item| Range {
                begin: item.time,                   // 印字できる時刻
                end: item.time + item.duration + 1, // 印字できなくなる時刻
            })
            .collect_vec();

        let range_group_by_begin: BTreeMap<usize, Vec<(usize, Range)>> = ranges
            .iter()
            .copied()
            .enumerate()
            .into_group_map_by(|(_, r)| r.begin)
            .into_iter()
            .collect::<BTreeMap<_, _>>();
        let mut current_time = 0;
        // 印字機の範囲にいるアイテムの(印字できなくなる時刻, 添字)
        let mut in_printer_set = BTreeSet::new();
        let mut ans = 0; // 印字したアイテムの数

        loop {
            // 印字機の範囲に入れる
            for (i, r) in range_group_by_begin.get(&current_time).unwrap_or(&vec![]) {
                in_printer_set.insert((r.end, *i));
            }
            // 印字できなくなる時刻が早い順番に取り出す貪欲
            if let Some((end, i)) = in_printer_set.range((current_time + 1, 0)..).next() {
                in_printer_set.remove(&(*end, *i));
                ans += 1;
                current_time += 1;
            } else {
                // 印字機の範囲にアイテムがない場合は、次のアイテムが印字機に入るまで時間を進める。
                if let Some(next_time) = range_group_by_begin
                    .range(current_time + 1..)
                    .next()
                    .map(|(i, _)| *i)
                {
                    current_time = next_time
                } else {
                    // アイテムがもうない
                    break;
                }
            }
        }

        Answer { ans }
    }

    fn solve2(&self) -> Answer {
        // Priority Queue ベースでシミュレーション (solve では BTreeMap でシミュレーションしていた)
        let items = self
            .items
            .iter()
            .copied()
            .sorted_by_key(|i| i.time)
            .collect_vec();

        let mut item_itr = items.iter().copied().peekable();

        let mut print_pq: BinaryHeap<Reverse<usize /* 期限切れの時刻 */>> = BinaryHeap::new();

        let mut cur_time = 0;
        let mut ans = 0;

        while item_itr.peek().is_some() || print_pq.peek().is_some() {
            // 期限切れを消す
            while print_pq
                .peek()
                .map(|Reverse(expired_time)| *expired_time <= cur_time)
                .unwrap_or(false)
            {
                print_pq.pop();
            }

            // print するものがなければ時間を進める
            if print_pq.peek().is_none() && item_itr.peek().is_some() {
                cur_time = item_itr.peek().unwrap().time;
            }

            // print_pq に詰める
            while item_itr.peek().map(|i| i.time <= cur_time).unwrap_or(false) {
                let item = item_itr.next().unwrap();
                print_pq.push(Reverse(item.time + item.duration + 1));
            }

            // print_pq から1つ print する
            if print_pq.pop().is_some() {
                ans += 1;
            }
            cur_time += 1;
        }

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
    Problem::read().solve2().print();
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

use std::{
    cmp::Reverse,
    collections::{BTreeMap, BTreeSet, BinaryHeap},
    time::Duration,
};

use itertools::enumerate;
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
