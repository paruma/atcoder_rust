#[allow(unused_imports)]
use myio::*;
pub mod myio {
    use std::io;

    pub fn read_line() -> String {
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).unwrap();
        buffer.trim().to_string()
    }

    pub fn read_vec_i64() -> Vec<i64> {
        let buf = read_line();
        buf.trim()
            .split(' ')
            .map(|s| s.parse::<i64>().unwrap())
            .collect::<Vec<i64>>()
    }

    pub fn read_vec_str() -> Vec<String> {
        let buf = read_line();
        buf.trim()
            .split(' ')
            .map(|s| s.to_string())
            .collect::<Vec<String>>()
    }

    pub fn read_i64_1() -> i64 {
        let buf = read_line();
        buf.parse::<i64>().unwrap()
    }

    pub fn read_i64_2() -> (i64, i64) {
        let ns = read_vec_i64();
        (ns[0], ns[1])
    }

    pub fn read_i64_3() -> (i64, i64, i64) {
        let ns = read_vec_i64();
        (ns[0], ns[1], ns[2])
    }

    pub fn read_i64_4() -> (i64, i64, i64, i64) {
        let ns = read_vec_i64();
        (ns[0], ns[1], ns[2], ns[3])
    }
}

fn solve(load_weight_list: &[i64], box_capacity_list: &[i64]) -> i64 {
    // box_capacity_list は昇順にソートされている。
    // load_weight_listもソートする
    let mut load_weight_list = Vec::from(load_weight_list);
    load_weight_list.sort();
    let mut cnt = 0;
    let mut box_capacity_list = box_capacity_list;

    for load_weight in load_weight_list {
        let box_info = box_capacity_list
            .iter()
            .enumerate()
            .find(|(_i, &capacity)| capacity >= load_weight);

        match box_info {
            Some((i, capacity)) => {
                assert!(box_capacity_list[i] == *capacity);
                box_capacity_list = &box_capacity_list[i + 1..];
                cnt += 1;
            }
            None => break,
        }
    }
    cnt
}

fn main() {
    let (n_loads, m_boxes) = read_i64_2();
    let load_weight_list = read_vec_i64();
    let box_capacity_list = read_vec_i64();

    let ans = solve(&load_weight_list, &box_capacity_list);

    println!("{}", ans);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {}
}
