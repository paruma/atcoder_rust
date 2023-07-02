use std::io;

fn solve(sentense: &str, target: char) -> Vec<usize> {
    let sentense = sentense.chars().collect::<Vec<char>>();
    sentense
        .iter()
        .enumerate()
        .filter(|(_, v)| **v == target)
        .map(|(i, _)| i)
        .take(10)
        .collect()
}

fn main() {
    let sentence = "Lorem ipsum dolor sit amet consectetur adipiscing elit sed do eiusmod tempor incididunt ut labore et dolore magna aliqua Ut enim ad minim veniam quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur Excepteur sint occaecat cupidatat non proident sunt in culpa qui officia deserunt mollit anim id est laborum";
    let ans = solve(sentence, 's');
    ans.iter().for_each(|n| println!("{}", n));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let s1 = "hoge";
        let s2 = "fuga";
        let s1_iter = s1.as_bytes().iter();
        let s2_iter = s2.as_bytes().iter();
        let zipped = s1_iter
            .zip(s2_iter)
            .map(|(&x, &y)| (x, y))
            .collect::<Vec<(u8, u8)>>();

        zipped
            .iter()
            .for_each(|(x, y)| println!("x={}, y={}", *x as char, *y as char));

        let s1_iter = s1.as_bytes().iter();
        let s2_iter = s2.as_bytes().iter();
        let cnt = s1_iter.zip(s2_iter).filter(|(x, y)| **x == **y).count();
        println!("{}", cnt);
    }
}
