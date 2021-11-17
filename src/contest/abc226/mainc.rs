#![allow(clippy::let_unit_value)]
use std::collections::VecDeque;

use itertools::Itertools;

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

#[allow(dead_code)]
fn solve(skills: &[Skill]) -> i64 {
    // 最初の実装、will_useを使わずにvisitedだけで実装しようとしたら事故った
    // 何があったんだっけ。
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

#[allow(dead_code)]
fn solve2(skills: &[Skill]) -> i64 {
    // 最初の実装、will_useを使わずにvisitedだけで実装しようとしたら事故った
    // 何があったんだっけ。
    let mut visited = vec![false; skills.len()];
    let mut open: VecDeque<usize> = VecDeque::new();

    open.push_front(skills.len() - 1);
    visited[skills.len() - 1] = true;

    // これwhile letで書けるけど、Rust限定ぽさがあるし、普通に書くことにする
    /*
        while let Some(current_idx)  = open.pop_back(){

        }
    */

    while !open.is_empty() {
        let current_idx = open.pop_back().unwrap();

        for &next_idx in &skills[current_idx].required {
            if !visited[next_idx] {
                visited[next_idx] = true;
                open.push_front(next_idx);
            }
        }
    }
    //0

    /*
        (0..skills.len())
        .map(|i| if will_use[i] { skills[i].time } else { 0 })
        .sum()
    */

    // zipは使いにくいから使わなくていいかな。

    (0..skills.len())
        .filter(|&i| visited[i])
        .map(|i| skills[i].time)
        .sum()
}

// --- solve3 ---

fn dfs(skills: &[Skill], visited: &mut [bool], current_idx: usize) {
    for &next_idx in &skills[current_idx].required {
        if !visited[next_idx] {
            visited[next_idx] = true;
            dfs(skills, visited, next_idx);
        }
    }
}
// DFSで解く
fn solve3(skills: &[Skill]) -> i64 {
    let mut visited = vec![false; skills.len()];

    let init_idx = skills.len() - 1;
    visited[init_idx] = true;
    dfs(skills, &mut visited, init_idx);

    (0..skills.len())
        .filter(|&i| visited[i])
        .map(|i| skills[i].time)
        .sum()
}

// ---
//fn output() {}

fn main() {
    let skills = read();
    let ans = solve3(&skills);
    //output();
    println!("{}", ans);
}
