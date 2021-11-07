#![allow(clippy::let_unit_value)]
use std::collections::VecDeque;

use itertools::Itertools;
use proconio::input;
use whiteread::Reader;

//------snippet------

//-------------------

#[derive(Clone, Debug, PartialEq, Eq)]
struct Skill {
    time: i64,
    required: Vec<usize>,
}

fn read() -> Vec<Skill> {
    let mut rdr = Reader::from_stdin_naive();
    let n = rdr.p::<usize>();

    (0..n)
        .map(|_| {
            let time = rdr.p::<i64>();
            let n_req = rdr.p::<usize>();
            let required = (0..n_req).map(|_| rdr.p::<usize>() - 1).collect_vec();
            Skill { time, required }
        })
        .collect_vec()
}

fn solve(skills: &[Skill]) -> i64 {
    let mut visited = vec![false; skills.len()];
    let mut will_use = vec![false; skills.len()];
    let mut open: VecDeque<usize> = VecDeque::new();

    open.push_front(skills.len() - 1);
    will_use[skills.len() - 1] = true;

    while !open.is_empty() {
        let current_idx = open.pop_back().unwrap();

        for &next_idx in &skills[current_idx].required {
            if !visited[next_idx] {
                visited[next_idx] = true;
                open.push_front(next_idx);
                will_use[next_idx] = true;
            }
        }
    }
    //0

    (0..skills.len())
        .map(|i| if will_use[i] { skills[i].time } else { 0 })
        .sum()
}

//fn output() {}

fn main() {
    let skills = read();
    let ans = solve(&skills);
    //output();
    println!("{}", ans);
}
