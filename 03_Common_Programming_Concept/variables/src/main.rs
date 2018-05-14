fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    // コンパイル時定数
    const MAX_POINTS: u32 = 100_000;
    println!("The value of MAX_POINTS is: {}", MAX_POINTS);

    // Shadowing
    let x = 5;
    let x = x + 1;
    let x = x * 2;
    println!("The value of x is: {}", x);

    let spaces = "   ";
    let spaces = spaces.len(); // 型が変わってもよい
    println!("The value of spaces is: {}", spaces);

    // コンパイルエラー
    // let mut spaces = "   ";
    // spaces = spaces.len(); 

    // コンパイルエラー
    // let guess = "42".parse().expect("Not a number!");
    
    let guess: u32 = "42".parse().expect("Not a number!");
    println!("The value of guess is: {}", guess);

    let x = 2.0; // f64
    let y: f32 = 3.0; // f32
    println!("{}, {}", x, y);

    // タプル型
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("{}, {}, {}", x, y, z);
    let x = tup.0;
    let y = tup.1;
    let z = tup.2;
    println!("{}, {}, {}", x, y, z);

    // 配列型
    let a = [1, 2, 3, 4, 5];
    let first = a[0];
    let second = a[1];
    println!("{}, {}", first, second);

    // 範囲外アクセスはパニックを引き起こす
    // let element = a[10];
}
