fn main() {
    let coin = Coin::Quarter(UsState::Alaska);
    value_in_cents(coin);

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    let some_u3_value = Some(3u8);
    
    match some_u3_value {
      Some(3) => println!("three"),
      _ => (),
    }

    // 上と同じ
    if let Some(3) = some_u3_value {
        println!("three");
    }
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // ... など
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarte from {:?}", state);
            25
        },
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
