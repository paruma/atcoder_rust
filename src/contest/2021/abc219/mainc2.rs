#![allow(clippy::let_unit_value)]
use itertools::Itertools;
use proconio::{input, marker::Chars};

//------snippet------

//-------------------

fn read() -> (Vec<char>, usize, Vec<Vec<char>>) {
    input! {
        x: Chars,
        n: usize,
        names: [Chars;n]
    }
    (x, n, names)
}

fn solve(x: &[char], _n: usize, names: &[Vec<char>]) -> Vec<String> {
    // 変換表
    let mut table: Vec<char> = vec![0 as char; 255]; // table['c']

    for (i, &c) in x.iter().enumerate() {
        let i = i as u8;
        table[c as usize] = (i + b'a') as char;
    }
    let mut names = names.to_vec();

    names.sort_by_key(|name| name.iter().map(|c| table[*c as usize]).collect::<String>());
    //names.sort_by_cached_key(f)

    // namesをVec<Char>のリストで持っていたから、Stringに変換しないといけない。
    names
        .iter()
        .map(|name| name.iter().collect::<String>())
        .collect_vec()
}

fn output(ans: &[String]) {
    for s in ans {
        println!("{}", s);
    }
}

fn main() {
    let (x, n, names) = read();
    let ans = solve(&x, n, &names);
    output(&ans);
    //println!("{}", ans);
}
