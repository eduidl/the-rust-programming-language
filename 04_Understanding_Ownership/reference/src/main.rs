fn main() {
    let s = String::from("hello");
    let len = calculate_length(&s);
    println!("The length of '{}' is {}.", s, len);

    let mut s = String::from("hello");
    change(&mut s);
    println!("s is {}", s);

    // 可変の借用を2つ以上同時にはできない. コンパイルエラー
    // let r1 = &mut s; // OK
    // let r2 = &mut s; // ERROR

    // 既に不変で借用されているときは、可変で借用できない. コンパイルエラー
    // let r1 = &s; // OK
    // let r2 = &s; // OK
    // let r3 = &mut s; // ERROR
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

// some_stringはimmutable. コンパイルエラー
// fn change(some_string: &String) {
//     some_string.push_str(", world");
// }

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

// s自体は関数を抜けた途端開放される. コンパイルエラー
// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s
// }

// Stringをそのまま返せばひとまず問題ない.
fn no_dangle() -> String {
    let s = String::from("hello");
    s
}