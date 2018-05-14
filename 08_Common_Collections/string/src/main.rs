fn main() {
    let s = "initial contents".to_string();
    // let s = String::from("initial contenst"); でも可

    let mut s = String::from("foo");
    let s2 = "bar";
    s.push_str(s2);
    // 所有権は失われない
    println!("s2 is {}", s2);

    let mut s = String::from("lo");
    s.push('l');

    let s1 = String::from("Hello, ");
    let s2 = String::from("World!");
    let s3 = s1 + &s2; // s1はムーブされる
    println!("s2 is {}, s3 is {}", s2, s3);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    // let s = s1 + "-" + &s2 + "-" + &s3;
    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("s is {}", s);

    // 添字アクセスは不可能
    // let h = s1[0];

    let hello = "いろは";
    println!("first char is {}, second char is {}", &hello[0..3], &hello[3..6]);
    // 文字の途中でスライスしようとするとコンパイルエラー
    // let invalid_slice = &hello[0..4];
    
    for c in "いろは".chars() {
        println!("{}", c);
    }

    for b in "いろは".bytes() {
        println!("{}", b);
    }
}
