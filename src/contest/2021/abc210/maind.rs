#![allow(clippy::let_unit_value)]
use std::collections::BinaryHeap;

use itertools::iproduct;
use ndarray::{Array, Array2};
use proconio::{input, source::once::OnceSource};

fn read() -> (usize, usize, i64, Array2<i64>) {
    input! {
        from OnceSource::from("3 4 2
        1 7 7 9
        9 6 3 7
        7 8 6 4
        "),
        h:usize, w: usize, c: i64,
        a: [[i64;w];h]
    }
    let table: Array2<i64> = Array::from_shape_fn((h, w), |(y, x)| a[y][x]);
    (h, w, c, table)
}

#[allow(dead_code)]
fn dist((y1, x1): (usize, usize), (y2, x2): (usize, usize)) -> i64 {
    let x1 = x1 as i64;
    let y1 = y1 as i64;
    let x2 = x2 as i64;
    let y2 = y2 as i64;

    (y1 - y2).abs() + (x1 - x2).abs()
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct HeapElem {
    y: usize,
    x: usize,
    score: i64,
}

impl PartialOrd for HeapElem {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        PartialOrd::partial_cmp(&self.score, &other.score).map(|c| c.reverse())
    }
}

impl Ord for HeapElem {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

fn solve(h: usize, w: usize, c: i64, table: &Array2<i64>) -> i64 {
    // (y,x,score)
    let mut p_queue: BinaryHeap<HeapElem> = BinaryHeap::new();
    for y in 0..h {
        for x in 0..w {
            p_queue.push(HeapElem {
                y,
                x,
                score: table[[y, x]],
            })
        }
    }
    let dir4 = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    let is_within = |y: i64, x: i64| {
        let h = h as i64;
        let w = w as i64;

        0 <= y && y < h && 0 <= x && x < w
    };

    let mut table2: Array2<i64> = table.clone();
    //table2の生成
    while !p_queue.is_empty() {
        let current = p_queue.pop().unwrap();
        // 更新の必要ない

        // >=って書いてバグらせてた
        if current.score > table2[[current.y, current.x]] {
            continue;
        }
        for (dy, dx) in dir4.iter() {
            let next_y = current.y as i64 + dy;
            let next_x = current.x as i64 + dx;
            if !is_within(next_y, next_x) {
                continue;
            }

            let next_y = next_y as usize;
            let next_x = next_x as usize;

            //table2の更新
            if table2[[current.y, current.x]] + c <= table2[[next_y, next_x]] {
                table2[[next_y, next_x]] = table2[[current.y, current.x]] + c;
                let next = HeapElem {
                    y: next_y,
                    x: next_x,
                    score: table2[[next_y, next_x]], //更新後
                };
                p_queue.push(next);
            };
        }
    }
    dbg!(table2);
    0
}

//愚直解
fn _solve2(h: usize, w: usize, c: i64, table: &Array2<i64>) -> i64 {
    iproduct!(0..h, 0..w, 0..h, 0..w)
        .filter(|(y1, x1, y2, x2)| (y1, x1) != (y2, x2))
        .map(|(y1, x1, y2, x2)| table[[y1, x1]] + table[[y2, x2]] + c * dist((y1, x1), (y2, x2)))
        .min()
        .unwrap()
}

// fn output() {}

// 未完成(アルゴリズム間違えてた)
fn main() {
    let (h, w, c, table) = read();
    let ans = solve(h, w, c, &table);
    //output();
    println!("{}", ans);
}
