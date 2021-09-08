// https://gist.github.com/jimmychu0807/9a89355e642afad0d2aeda52e6ad2424

fn main() {
    // -- FROM: vec of chars --
    let src1: Vec<char> = vec!['j', '{', '"', 'i', 'm', 'm', 'y', '"', '}'];
    // to String
    let string1: String = src1.iter().collect::<String>();
    // to str
    // これ実体死んでない？参照取って大丈夫なの？
    let str1: &str = &src1.iter().collect::<String>();
    // to vec of byte
    let byte1: Vec<u8> = src1.iter().map(|c| *c as u8).collect::<Vec<_>>();
    println!(
        "Vec<char>:{:?} | String:{:?}, str:{:?}, Vec<u8>:{:?}",
        src1, string1, str1, byte1
    );

    // -- FROM: vec of bytes --
    // in rust, this is a slice
    // b - byte, r - raw string, br - byte of raw string
    let src2: Vec<u8> = br#"e{"ddie"}"#.to_vec();
    // to String
    // from_utf8 consume the vector of bytes
    let string2: String = String::from_utf8(src2.clone()).unwrap();

    //a value of type `std::string::String` cannot be built from an iterator over elements of type `&u8`
    //the trait `std::iter::FromIterator<&u8>` is not implemented for `std::string::String`

    //impl<'a> FromIterator<&'a char> for String
    // はあるけど
    // impl<'a> FromIterator<&'a u8> for String
    // はない
    // let string2: String = src2.iter().collect::<String>();

    // to str
    let str2: &str = std::str::from_utf8(&src2).unwrap(); // これってsrc2がdropしたら使えなくなりそう。なった。

    // to vec of chars
    let char2: Vec<char> = src2.iter().map(|&b| b as char).collect::<Vec<_>>();
    println!(
        "Vec<u8>:{:?} | String:{:?}, str:{:?}, Vec<char>:{:?}",
        src2, string2, str2, char2
    );

    // -- FROM: String --
    let src3: String = String::from(r#"o{"livia"}"#);
    let str3: &str = &src3;
    let char3: Vec<char> = src3.chars().collect();
    let byte3: Vec<u8> = src3.as_bytes().to_vec(); // ↑と同様にsrc3.bytes().collect()でも良い
    println!(
        "String:{:?} | str:{:?}, Vec<char>:{:?}, Vec<u8>:{:?}",
        src3, str3, char3, byte3
    );

    // -- FROM: str --
    let src4: &str = r#"g{'race'}"#;
    let string4 = String::from(src4);
    let char4: Vec<char> = src4.chars().collect();
    let byte4: Vec<u8> = src4.as_bytes().to_vec();
    println!(
        "str:{:?} | String:{:?}, Vec<char>:{:?}, Vec<u8>:{:?}",
        src4, string4, char4, byte4
    );

    //--------------------------//

    let _s1 = String::from("hoge");
    let _s2 = &_s1;

    let _x = &1;
    let _y = *_x;

    // このString::from("hoge");っていつdropするの？
    let _test: &String = &String::from("hoge");

    let _test2 = _test;
    println!("{}", _test);

    _sub();
}

fn _sub() {
    let s1: String = String::from("hoge");

    let s2: &str = "hoge";

    let chars: Vec<char> = s2.chars().collect::<Vec<char>>();

    //let bytes = s2.as_bytes().to_vec();

    let bytes: Vec<u8> = s2.bytes().collect::<Vec<u8>>();

    println!("{:?}{:?}{:?}{:?}", &s1, s2, &chars, &bytes);


    let hoge = vec![1,2,3];
    let hoge:&[i32] = &hoge;
    let _hoge = hoge.to_vec();
}

fn _sub2() {
    // これ死ぬ。
    /*
    let x = {
        let src2: Vec<u8> = br#"e{"ddie"}"#.to_vec();
        let str2: &str = std::str::from_utf8(&src2).unwrap(); // これってsrc2がdropしたら使えなくなりそう。
        str2
    };
    */
}
