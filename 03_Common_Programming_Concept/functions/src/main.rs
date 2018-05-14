fn main() {
    another_function(5, 6);

    // コンパイルエラー: letは文
    // let x = (let y = 6);

    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of y is: {}", y);

    let x = five();
    println!("The value of x is: {}", x);
}

fn another_function(x: i32, y:i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

fn five() -> i32 {
    5
    // 5;
    // のようにセミコロンを書くと文になり、返り値がなくなるためコンパイルエラー
}
