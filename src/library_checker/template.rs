fn main() {
    input! {
        a: i64,
        b: i64,
    }
    let ans = a + b;
    println!("{}", ans)
}

// ------------------入力------------------
// https://qiita.com/tanakh/items/0ba42c7ca36cd29d0ac8 からコピー
#[macro_export]
macro_rules! input {
    (source = $s:expr_2021, $($r:tt)*) => {
        let mut iter = $s.split_whitespace();
        let mut next = || { iter.next().unwrap() };
        input_inner!{next, $($r)*}
    };
    ($($r:tt)*) => {
        let stdin = std::io::stdin();
        let mut bytes = std::io::Read::bytes(std::io::BufReader::new(stdin.lock()));
        let mut next = move || -> String{
            bytes
                .by_ref()
                .map(|r|r.unwrap() as char)
                .skip_while(|c|c.is_whitespace())
                .take_while(|c|!c.is_whitespace())
                .collect()
        };
        input_inner!{next, $($r)*}
    };
}

#[macro_export]
macro_rules! input_inner {
    ($next:expr_2021) => {};
    ($next:expr_2021, ) => {};

    ($next:expr_2021, $var:ident : $t:tt $($r:tt)*) => {
        let $var = read_value!($next, $t);
        input_inner!{$next $($r)*}
    };
}

#[macro_export]
macro_rules! read_value {
    ($next:expr_2021, ( $($t:tt),* )) => {
        ( $(read_value!($next, $t)),* )
    };

    ($next:expr_2021, [ $t:tt ; $len:expr_2021 ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    };

    ($next:expr_2021, chars) => {
        read_value!($next, String).chars().collect::<Vec<char>>()
    };

    ($next:expr_2021, usize1) => {
        read_value!($next, usize) - 1
    };

    ($next:expr_2021, $t:ty) => {
        $next().parse::<$t>().expect("Parse error")
    };
}
