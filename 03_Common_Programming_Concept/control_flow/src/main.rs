fn main() {
    let condition = true;
    let number = if condition {
        5
    } else {
        6
    };
    println!("the value of number is: {}", number);

    // 型が揃っていないためコンパイルエラー
    // let number = if condition { 5 } else { "six" };
    
    // 無限ループ
    loop { 
        println!("loop");
        break
    }

    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }

    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {}", element);
    }

    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}
