#[test]
fn ex_gcd() {
    //fn gcd<T: Integer>(x: T, y: T) -> T
    assert_eq!(num_integer::gcd(4, 6), 2);
    assert_eq!(num_integer::lcm(4, 6), 12);

    assert_eq!(num_integer::gcd(0, 6), 6);
    assert_eq!(num_integer::gcd(0, 0), 0);
}

#[test]
fn ex_binomial() {
    assert_eq!(num_integer::binomial(5, 2), 10);
    assert_eq!(num_integer::multinomial(&[3, 2]), 10); // (3 + 2)!/(3! * 2!)
}

#[test]
fn ex_divceil_divfloor() {
    assert_eq!(num_integer::div_ceil(8, 3), 3);
    // 7/3の代わりに使っていく
    assert_eq!(num_integer::div_floor(8, 3), 2);
    assert_eq!(num_integer::div_ceil(-8, 3), -2);
    assert_eq!(num_integer::div_floor(-8, 3), -3);

    // これは標準ライブラリ
    assert_eq!(8_i32.div_euclid(3), 2);
    assert_eq!((-8_i32).div_euclid(3), -3);
    assert_eq!(i32::div_euclid(8, 3), 2);
    assert_eq!(i32::div_euclid(-8, 3), -3);
}

#[test]
fn ex_mod() {
    // これは標準ライブラリ(num_integerにはない)
    assert_eq!(8_i32.rem_euclid(3), 2); // 8 = 2 * 3 + 2
    assert_eq!((-8_i32).rem_euclid(3), 1); // -8 = (-3) * 3 + 1
    assert_eq!(i32::rem_euclid(8, 3), 2);
    assert_eq!(i32::rem_euclid(-8, 3), 1);
}
/*
i32系で重要そうな関数

* `abs`
* `div_euclid`, `rem_euclid`
* `pow`
* `sig_num`
 */

#[test]
fn ex_sqrt() {
    //---浮動小数点---
    let x: f64 = 3.2;
    // xの型を明示的に与える必要がある。
    // f64::sqrtかf32:sqrtのどっちを呼べばいいかわからないため。
    let _y: f64 = x.sqrt();
    let _y: f64 = f64::sqrt(3.2);
    let _y: f64 = num::Float::sqrt(x); // num::Float: f32, f64を統一的に扱う

    //---整数---
    assert_eq!(num_integer::sqrt(5), 2);
    assert_eq!(num_integer::sqrt(4), 2);
    assert_eq!(num_integer::sqrt(3), 1);
}
