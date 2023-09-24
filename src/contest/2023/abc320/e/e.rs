use std::{cmp::Reverse, collections::BinaryHeap, io::stdin};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct SoumenEvent {
    time: i64,
    amount: i64, // 流すそうめんの量
    len: i64,    //そうめんを取ってからもとに戻るまでの時間
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct ComeBackEvent {
    person: usize,
    time: i64, // 戻ってくる時刻
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Event {
    Soumen(SoumenEvent),
    ComeBack(ComeBackEvent), // 同時刻の場合はこっちが優先
}

impl Event {
    fn get_time(&self) -> i64 {
        match self {
            Event::Soumen(e) => e.time,
            Event::ComeBack(e) => e.time,
        }
    }
}

impl PartialOrd for Event {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        // 時間の小さい順にBinaryHeapが出てくるようにする
        // Soumen と ComeBack が同じ場合は ComeBack を優先する
        if self.get_time() == other.get_time() {
            return match self {
                Event::Soumen(_) => Some(std::cmp::Ordering::Less),
                Event::ComeBack(_) => Some(std::cmp::Ordering::Greater), // ComeBack が優先
            };
        }
        PartialOrd::partial_cmp(&self.get_time(), &other.get_time()).map(|c| c.reverse())
    }

    // (get_time(), 1(Soumen) or 0(ComeBack)) で辞書順比較をするのもありかも
}

impl Ord for Event {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

struct Problem {
    n_people: usize,
    n_event: usize,
    events: Vec<SoumenEvent>,
}

impl Problem {
    fn read<R: IProconReader>(mut r: R) -> Problem {
        let (n_people, n_event) = r.read_usize_2();
        let events = (0..n_event)
            .map(|_| {
                let (time, amount, len) = r.read_i64_3();
                SoumenEvent { time, amount, len }
            })
            .collect_vec();
        Problem { n_people, n_event, events }
    }
    fn solve(&self) -> Answer {
        // リファクタリング内容
        //*  */ Reverse を使った(自前で順序逆転の構造体を作る必要はなかった)
        // * `let Some(top_person)` → `let Some(Reverse(top_person))`
        let Problem { n_people, n_event, events } = self;
        let mut event_q_queue: BinaryHeap<Event> = BinaryHeap::new();

        for soumen_event in events {
            event_q_queue.push(Event::Soumen(*soumen_event));
        }

        let mut people_to_somen = vec![0; self.n_people];
        let mut waiting_people: BinaryHeap<Reverse<usize>> = BinaryHeap::new(); // そうめんの列に並んでいる人
        for i in 0..*n_people {
            waiting_people.push(Reverse(i))
        }

        while let Some(event) = event_q_queue.pop() {
            match event {
                Event::Soumen(e) => {
                    if let Some(Reverse(top_person)) = waiting_people.pop() {
                        people_to_somen[top_person] += e.amount;
                        event_q_queue.push(Event::ComeBack(ComeBackEvent {
                            person: top_person,
                            time: e.time + e.len,
                        }));
                    }
                }
                Event::ComeBack(e) => waiting_people.push(Reverse(e.person)),
            }
        }
        Answer { ans: people_to_somen }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Answer {
    ans: Vec<i64>,
}

impl Answer {
    fn print(&self) {
        for &x in &self.ans {
            println!("{}", x);
        }
    }
}

fn main() {
    Problem::read(ProconReader::new(stdin().lock())).solve().print();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(dead_code)]
    fn check(input: &str, expected: Answer) {
        let actual = Problem::read(ProconReader::new(input.as_bytes())).solve();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_problem() {
        let _input = "
3
4
        "
        .trim();
        // check(_input, Answer { ans: 7 });
    }
}

// ====== snippet ======

use itertools::Itertools;
#[allow(unused_imports)]
use myio::*;
pub mod myio {
    use std::io::BufRead;

    pub trait IProconReader {
        fn read_line(&mut self) -> String;

        fn read_bytes(&mut self) -> Vec<u8> {
            self.read_line().as_bytes().to_vec()
        }

        fn read_any_1<T>(&mut self) -> T
        where
            T: std::str::FromStr,
            T::Err: std::fmt::Debug,
        {
            let buf = self.read_line();
            buf.parse::<T>().unwrap()
        }

        fn read_any_2<T0, T1>(&mut self) -> (T0, T1)
        where
            T0: std::str::FromStr,
            T0::Err: std::fmt::Debug,
            T1: std::str::FromStr,
            T1::Err: std::fmt::Debug,
        {
            let buf = self.read_line();
            let splitted = buf.trim().split(' ').collect::<Vec<_>>();
            let a0 = splitted[0].parse::<T0>().unwrap();
            let a1 = splitted[1].parse::<T1>().unwrap();
            (a0, a1)
        }

        fn read_any_3<T0, T1, T2>(&mut self) -> (T0, T1, T2)
        where
            T0: std::str::FromStr,
            T0::Err: std::fmt::Debug,
            T1: std::str::FromStr,
            T1::Err: std::fmt::Debug,
            T2: std::str::FromStr,
            T2::Err: std::fmt::Debug,
        {
            let buf = self.read_line();
            let splitted = buf.trim().split(' ').collect::<Vec<_>>();
            let a0 = splitted[0].parse::<T0>().unwrap();
            let a1 = splitted[1].parse::<T1>().unwrap();
            let a2 = splitted[2].parse::<T2>().unwrap();
            (a0, a1, a2)
        }

        fn read_any_4<T0, T1, T2, T3>(&mut self) -> (T0, T1, T2, T3)
        where
            T0: std::str::FromStr,
            T0::Err: std::fmt::Debug,
            T1: std::str::FromStr,
            T1::Err: std::fmt::Debug,
            T2: std::str::FromStr,
            T2::Err: std::fmt::Debug,
            T3: std::str::FromStr,
            T3::Err: std::fmt::Debug,
        {
            let buf = self.read_line();
            let splitted = buf.trim().split(' ').collect::<Vec<_>>();
            let a0 = splitted[0].parse::<T0>().unwrap();
            let a1 = splitted[1].parse::<T1>().unwrap();
            let a2 = splitted[2].parse::<T2>().unwrap();
            let a3 = splitted[3].parse::<T3>().unwrap();
            (a0, a1, a2, a3)
        }
        fn read_vec_any<T>(&mut self) -> Vec<T>
        where
            T: std::str::FromStr,
            T::Err: std::fmt::Debug,
        {
            let buf = self.read_line();
            buf.trim().split(' ').map(|s| s.parse::<T>().unwrap()).collect::<Vec<T>>()
        }

        fn read_vec_i64(&mut self) -> Vec<i64> {
            self.read_vec_any::<i64>()
        }

        fn read_vec_usize(&mut self) -> Vec<usize> {
            self.read_vec_any::<usize>()
        }

        fn read_vec_str(&mut self) -> Vec<String> {
            self.read_vec_any::<String>()
        }

        fn read_i64_1(&mut self) -> i64 {
            self.read_any_1::<i64>()
        }

        fn read_i64_2(&mut self) -> (i64, i64) {
            self.read_any_2::<i64, i64>()
        }

        fn read_i64_3(&mut self) -> (i64, i64, i64) {
            self.read_any_3::<i64, i64, i64>()
        }

        fn read_i64_4(&mut self) -> (i64, i64, i64, i64) {
            self.read_any_4::<i64, i64, i64, i64>()
        }

        fn read_usize_1(&mut self) -> usize {
            self.read_any_1::<usize>()
        }

        fn read_usize_2(&mut self) -> (usize, usize) {
            self.read_any_2::<usize, usize>()
        }

        fn read_usize_3(&mut self) -> (usize, usize, usize) {
            self.read_any_3::<usize, usize, usize>()
        }

        fn read_usize_4(&mut self) -> (usize, usize, usize, usize) {
            self.read_any_4::<usize, usize, usize, usize>()
        }
    }

    pub struct ProconReader<R: BufRead> {
        buf_read: R,
    }

    impl<R: BufRead> ProconReader<R> {
        pub fn new(buf_read: R) -> ProconReader<R> {
            ProconReader { buf_read }
        }
    }

    impl<R: BufRead> IProconReader for ProconReader<R> {
        fn read_line(&mut self) -> String {
            let mut buffer = String::new();
            self.buf_read.read_line(&mut buffer).unwrap();
            buffer.trim().to_string()
        }
    }
}
