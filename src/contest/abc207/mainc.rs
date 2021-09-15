#![allow(clippy::let_unit_value)]
use itertools::Itertools;
use proconio::input;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Interval {
    ClCl { begin: i64, end: i64 },
    ClOp { begin: i64, end: i64 },
    OpCl { begin: i64, end: i64 },
    OpOp { begin: i64, end: i64 },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum IntClOp {
    Cl { n: i64 },
    Op { n: i64 },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Interval2 {
    begin: IntClOp,
    end: IntClOp,
}

fn read() -> (usize, Vec<Interval>) {
    input! {
        n: usize,
        infos: [(i64, i64, i64); n]
    }
    let intervals = infos
        .into_iter()
        .map(|(type_no, begin, end)| match type_no {
            1 => Interval::ClCl { begin, end },
            2 => Interval::ClOp { begin, end },
            3 => Interval::OpCl { begin, end },
            4 => Interval::OpOp { begin, end },
            _ => panic!(),
        })
        .collect::<Vec<_>>();

    (n, intervals)
}

fn convert(interval: Interval) -> Interval2 {
    match interval {
        Interval::ClCl { begin, end } => Interval2 {
            begin: IntClOp::Cl { n: begin },
            end: IntClOp::Cl { n: end },
        },
        Interval::ClOp { begin, end } => Interval2 {
            begin: IntClOp::Cl { n: begin },
            end: IntClOp::Op { n: end },
        },
        Interval::OpCl { begin, end } => Interval2 {
            begin: IntClOp::Op { n: begin },
            end: IntClOp::Cl { n: end },
        },
        Interval::OpOp { begin, end } => Interval2 {
            begin: IntClOp::Op { n: begin },
            end: IntClOp::Op { n: end },
        },
    }
}
fn intersects(interval1: Interval2, interval2: Interval2) -> bool {
    let check1: bool = {
        match (interval1.end, interval2.begin) {
            (IntClOp::Cl { n: end1 }, IntClOp::Cl { n: begin2 }) => end1 < begin2,
            (IntClOp::Cl { n: end1 }, IntClOp::Op { n: begin2 }) => end1 <= begin2,
            (IntClOp::Op { n: end1 }, IntClOp::Cl { n: begin2 }) => end1 <= begin2,
            (IntClOp::Op { n: end1 }, IntClOp::Op { n: begin2 }) => end1 <= begin2,
        }
    };

    let check2: bool = {
        match (interval2.end, interval1.begin) {
            (IntClOp::Cl { n: end2 }, IntClOp::Cl { n: begin1 }) => end2 < begin1,
            (IntClOp::Cl { n: end2 }, IntClOp::Op { n: begin1 }) => end2 <= begin1,
            (IntClOp::Op { n: end2 }, IntClOp::Cl { n: begin1 }) => end2 <= begin1,
            (IntClOp::Op { n: end2 }, IntClOp::Op { n: begin1 }) => end2 <= begin1,
        }
    };
    !(check1 || check2)
}

fn solve(n: usize, intervals: &[Interval]) -> i64 {
    (0..n)
        .tuple_combinations()
        .filter(|(i1, i2)| {
            let interval1 = convert(intervals[*i1]);
            let interval2 = convert(intervals[*i2]);

            intersects(interval1, interval2)
        })
        .count() as i64
}

//fn output() {}

fn main() {
    let (n, intervals) = read();
    let ans = solve(n, &intervals);
    println!("{}", ans);
}
