#![allow(clippy::let_unit_value)]
use itertools::Itertools;
use proconio::{input, marker::Usize1};

//------snippet------

//-------------------

fn read() -> (usize, usize, Vec<(i64, i64, i64)>) {
    input! {
        //from OnceSource::from(""),
        n: usize,k:Usize1, students: [(i64,i64,i64);n]
    }
    (n, k, students)
}

fn solve(n: usize, k: usize, students: &[(i64, i64, i64)]) -> Vec<bool> {
    let students = students
        .iter()
        .map(|(p1, p2, p3)| *p1 + *p2 + *p3)
        .collect_vec();

    // 降順
    let students_sorted = students.iter().sorted_by_key(|&&x| -x).collect_vec();

    let border = *students_sorted[k] - 300;

    students.iter().map(|x| *x >= border).collect_vec()
}

fn output(ans: &[bool]) {
    for e in ans {
        println!("{}", if *e { "Yes" } else { "No" })
    }
}

fn main() {
    let (n, k, students) = read();
    let ans = solve(n, k, &students);
    output(&ans);
    //println!("{}", ans);
}
