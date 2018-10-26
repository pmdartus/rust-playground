use std::convert::From;

#[derive(Debug)]
struct Number {
    value: i32,
}

impl From<i32> for Number {
    fn from(value: i32) -> Self {
        Number { value }
    }
}

fn main() {
    let num = Number::from(32);
    println!("{:?}", num);

    let int = 5;
    let another_num: Number = int.into();
    println!("{:?}", another_num);
}