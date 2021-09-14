#[allow(unused_macros)]
macro_rules! odo {
        (let $p: pat = $e: expr ; $($t:tt)+)=>{
            { let $p = $e ; odo! {$($t)+} }
        };
        (let $p: ident : $ty: ty = $e: expr ; $($t:tt)+)=>{
            { let $p: $ty = $e ; odo! {$($t)+} }
        };
        ($i:ident <- $e:expr; $($t:tt)+) => {
            $e.and_then(move|$i| odo!{$($t)+})
        };
        ($e:expr; $($t:tt)+) => {
            $e.and_then(move|_| odo!{$($t)+})
        };
        (guard $e:expr; $($t:tt)+)=>{
            ($e).then(move|| odo!{$($t)+}).flatten()
        };
        ($e:expr) => {
            $e
        };
}

#[test]
fn feature() {
    let z = odo! {
        x<- Some(3);
        y<- Some(4);
        Some(x + y)
    };
    assert_eq!(z, Some(7));
}
