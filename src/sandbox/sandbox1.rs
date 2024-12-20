// Rustでもモナドは実装できるのか？（再） - Don't Repeat Yourself https://blog-dry.com/entry/2020/12/25/130250

#[allow(unused_macros)]
macro_rules! mdo {
    ($i:ident <- $e:expr; $($t:tt)+) => {
        $e.and_then(|$i| mdo!($($t)+))
    };
    ($e:expr; $($t:tt)+) => {
        $e.and_then(|()| mdo!($($t)+))
    };
    ($e:expr) => {
        $e
    };
}

#[test]
fn _sandbox1() {
    let a_opt = Some(3);
    let b_opt = Some(2);

    let _a_vec = vec![1, 2, 3];
    let _b_vec = vec![4, 5, 6];

    let x = a_opt.and_then(|a| b_opt.map(|b| a + b));

    let y = mdo! {
        a <- a_opt;
        b <- b_opt;
        Some(a+b)
    };
    println!("{:?}, {:?}", &x, &y);
}

#[test]
fn _sandbox2() {
    let x = vec![Some(1), Some(3), None];
    let y = x.into_iter().flatten().collect::<Vec<i32>>();
    let _z = y;
}

#[allow(clippy::ptr_arg)]
fn _test(v: &Vec<i32>) {
    println!("{}", v.len());
}

#[allow(clippy::needless_borrow)]
#[test]
fn _sandbox4() {
    let v = vec![1, 2];
    v.len(); //これは(&v).len()と書いても同じ？
    (&v).len();

    _test(&v);

    // 借用しているだけで普通に使える。
    let x = &v;
    // let y: Vec<i32> = *x;// これはエラーか。Copyできないからエラーみたい。（所有者が2人になってしまう）
    println!("{}", x.len());

    let x = 10;
    let y = &x;
    let _z = *y; // i32はCopy可能
}
#[allow(dead_code)]
struct Hoge {}

impl Hoge {
    #[allow(dead_code)]
    fn hoge(&self) {}
}

#[allow(clippy::needless_borrow)]
#[test]
fn _sandbox5() {
    let x: Hoge = Hoge {};
    x.hoge();
    (&x).hoge();
    (&&x).hoge();
    // Hoge::hoge(x); //エラー: add reference here
    Hoge::hoge(&x);
    Hoge::hoge(&&x);
}
#[test]
fn _sandbox6() {
    let a = [1, 2, 3];
    let _sum: i32 = a.iter().sum();
}

// numトレイトのテスト
#[test]
fn _sandbox7() {
    assert_eq!(num::pow::<i32>(3, 5), 243);
}

fn _div2(x: i32) -> Option<i32> {
    //(i32::rem_euclid(x, 2) == 0).then(|| x / 2)
    if i32::rem_euclid(x, 2) == 0 {
        Some(x / 2)
    } else {
        None
    }
}

#[test]
fn _sandbox8() {
    let x1 = Some(8);
    let x2 = Some(6);

    // Haskellだとand_thenは >>=
    let y1 = x1.and_then(_div2).and_then(_div2);
    let y2 = x2.and_then(_div2).and_then(_div2);

    assert_eq!(y1, Some(2));
    assert_eq!(y2, None);
}

#[test]
fn _sandbox9() {
    let a_opt = Some(3);
    let b_opt = Some(4);
    let c_opt: Option<i32> = None;

    let ans_opt1 = (|| -> Option<i32> {
        let a = a_opt?;
        let b = b_opt?;
        Some(a + b)
    })();

    let ans_opt2 = (|| -> Option<i32> {
        let a = a_opt?;
        let c = c_opt?;
        Some(a + c)
    })();

    assert_eq!(ans_opt1, Some(7));
    assert_eq!(ans_opt2, None);
}
