fn main() {
    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    let third: &i32 = &v[2];
    println!("third is {}", third);
    let third: Option<&i32> = v.get(2);
    println!("third is {:?}", third);

    //  パニック
    // let dose_not_exist = &v[100];
    let dose_not_exist = v.get(100);

    println!("dose_not_exist is {:?}", dose_not_exist);

    // 借用されているためコンパイルエラー
    // v.push(6);

    for i in &v {
        println!("{}", i);
    }

    let mut v = vec![100, 32, 57];
    for i in &mut v {
        // *: 参照外し演算子
        *i += 50;
        println!("{}", i)
    }

    let row = vec![
        SpreadSheetCell::Int(3),
        SpreadSheetCell::Text(String::from("blue")),
        SpreadSheetCell::Float(10.12),
    ];
    println!("{:?}", row);
}

#[derive(Debug)]
enum SpreadSheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}
