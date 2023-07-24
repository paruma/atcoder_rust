#![allow(clippy::let_unit_value)]
use proconio::{input, marker::Usize1};

//------snippet------

//-------------------

fn read() -> (usize, usize, Vec<Vec<usize>>) {
    input! {
        //from OnceSource::from(""),
        height:usize, width: usize,
        cal: [[Usize1; width]; height]
    }
    (height, width, cal)
}

//TODO: 足し算できるのほしい
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Pos {
    x: usize,
    y: usize,
}

fn calc_pos(v: usize) -> Pos {
    let x = v % 7;
    let y = v / 7;
    Pos { x, y }
}
#[allow(clippy::needless_range_loop)]
fn solve(height: usize, width: usize, cal: &[Vec<usize>]) -> bool {
    // -1した値で考える
    let init_pos = calc_pos(cal[0][0]);

    for y in 0..height {
        for x in 0..width {
            let excepted_pos = Pos {
                x: x + init_pos.x,
                y: y + init_pos.y,
            };
            let real_pos = calc_pos(cal[y][x]);
            if excepted_pos != real_pos {
                return false;
            }
        }
    }

    true
}

fn main() {
    let (height, width, cal) = read();
    let ans = solve(height, width, &cal);
    let ans_str = if ans { "Yes" } else { "No" };
    //output();
    println!("{}", ans_str);
}
