#![allow(clippy::let_unit_value)]
use proconio::input;

fn read() -> (i64, i64, Vec<i64>) {
    input! {n_items:i64, money:i64, item_prices:[i64;n_items]}
    (n_items, money, item_prices)
}

fn solve(n_items: i64, money: i64, item_prices: &[i64]) -> bool {
    // 3つなら1円引き
    // 4つなら2円引き
    // 5つなら2円引き

    let sum_price = item_prices.iter().sum::<i64>();
    let discount = num_integer::div_floor(n_items, 2);
    let actual_price = sum_price - discount;
    actual_price <= money
}

fn output(ans: bool) {
    let msg = if ans { "Yes" } else { "No" };

    println!("{}", msg);
}

fn main() {
    let (n_items, money, item_prices)= read();
    let ans = solve(n_items, money, &item_prices);
    output(ans);
}
