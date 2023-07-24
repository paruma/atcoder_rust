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
    let mut ans: Vec<char> = Vec::new();
    for &ti in t {
        match ti {
            '1' => {
                let mut x = s1.to_vec();
                ans.append(&mut x)
            }
            '2' => {
                let mut x = s2.to_vec();
                ans.append(&mut x)
            }
            '3' => {
                let mut x = s3.to_vec();
                ans.append(&mut x)
            }
            _ => panic!(),
        }
    }
    ans.iter().collect::<String>()
}

//fn output() {}

fn main() {
    let (s1, s2, s3, t) = read();
    let ans = solve(&s1, &s2, &s3, &t);
    //output();
    println!("{}", ans);
}
