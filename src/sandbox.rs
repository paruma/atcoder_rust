// Rustでもモナドは実装できるのか？（再） - Don't Repeat Yourself https://blog-dry.com/entry/2020/12/25/130250
macro_rules! mdo {
    ($i:ident <- $e:expr; $($t:tt)*) => {
        $e.and_then(move |$i| mdo!($($t)*))
    };
    ($e:expr; $($t:tt)*) => {
        $e.and_then(move |()| mdo!($($t)*))
    };
    (ret $e:expr) => {
        $e
    };
}

#[allow(dead_code)]
fn sandbox1() {
    let a_opt = Some(3);
    let b_opt = Some(2);

    let a_vec = vec![1, 2, 3];
    let b_vec = vec![4, 5, 6];

    let x = a_opt.and_then(|a| b_opt.map(|b| a + b));

    let y = mdo! {
        a <- a_opt;
        b <- b_opt;
        ret Some(a+b)
    };
    println!("{:?}, {:?}", &x, &y);
}

fn sandbox2() {
    let x = vec![Some(1), Some(3), None];
    let y = x.into_iter().flatten().collect::<Vec<i32>>();
    let z = y;
}


mod test;
fn sandbox3(){
    test::test();
}

fn main() {
    sandbox2();
}
