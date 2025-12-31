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
/*
fn read() -> (i64, i64, i64, i64) {
    read_i64_4()
}*/

fn main() {
    let (n, a, b) = read_i64_3();
    let mut b = b as f64;
    let mut a = a as f64;
    let ans = std::iter::from_fn(move || {
        let ret = a + b;
        b = (a + b) / 2.0;
        Some(ret)
    })
    .take(n);
    ans.for_each(|x| println!("{}", x));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {}
}
