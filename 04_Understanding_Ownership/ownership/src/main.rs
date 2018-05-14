fn main() {
    let s1 = String::from("hello");
    let s2 = s1; // ムーブ

    // コンパイルエラー
    // println!("{}, world!", s);
    println!("{}, world!", s2);

    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);

    let x = 5;
    let y = x; // コピー
    println!("x = {}, y = {}", x, y);

    // Copy
    // 整数型, bool, 浮動小数点型, char
    // 上記の組み合わせによるタプル

    let s = String::from("hello");
    takes_ownership(s);
    // コンパイルエラー
    // println!("{}", s);

    let x = 5;
    makes_copy(x);
    println!("{}", x);

    let s1 = gives_ownership();
    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2);
    println!("s1 = {}, s3 = {}", s1, s3);
    // コンパイルエラー
    // println!("s2 = {}", s2);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
} // ここでsome_stringがスコープを抜け、`drop`が呼ばれる。

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}
