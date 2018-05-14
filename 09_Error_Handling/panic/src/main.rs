use std::io;
use std::io::Read;
use std::io::ErrorKind;
use std::fs::File;

fn main() {
    // panic!("crash and burn");

    let v = vec![1, 2, 3];
    // v[99];

    let f = File::open("hello.txt"); // Result<T, E>型

    let f = match f {
        Ok(file) => file,
        Err(ref error) if error.kind() == ErrorKind::NotFound => {
            match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => {
                    panic!("Tried to create file but there was a problem: {:?}", e)
                }
            }
        }
        Err(error) => {
            panic!("There was a problem opening the file: {:?}", error)
        },
    };

    let f = File::open("hello2.txt").unwrap();
    // エラーメッセージを指定可能
    // let f = File::open("hello2.txt").expect("Failed to open hello2.txt");
}

fn read_user_name_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

// 上と大体同じ
// ?はエラー型の型変換を行う
// Result<T, E>を返す関数内でしか呼べない
fn read_user_name_from_file_simple() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}