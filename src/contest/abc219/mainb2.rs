#![allow(clippy::let_unit_value)]
use proconio::{input, marker::Chars};

//------snippet------

//-------------------

fn read() -> (Vec<char>, Vec<char>, Vec<char>, Vec<char>) {
    input! {
        //from OnceSource::from(""),
        s1: Chars,
        s2: Chars,
        s3: Chars,
        t: Chars
    }
    (s1, s2, s3, t)
}

fn solve(s1: &[char], s2: &[char], s3: &[char], t: &[char]) -> String {
    let ss = [s1, s2, s3];
    t.iter()
        .map(|ti| {
            let idx = (*ti as usize) - ('1' as usize);
            ss[idx].to_vec()
        })
        .flatten()
        .collect::<String>()
}

//fn output() {}

fn main() {
    let (s1, s2, s3, t) = read();
    let ans = solve(&s1, &s2, &s3, &t);
    //output();
    println!("{}", ans);
}
