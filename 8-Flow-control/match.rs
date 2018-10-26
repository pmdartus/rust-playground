fn main() {
    let number = 1;

    match number {
        1 => println!("ONE!"),
        2 | 3 | 5 | 7 | 11 => println!("Prime {}", number),
        _ => println!("Other")
    }

    let pair = (1, -2);

    match pair {
        (0, y) => println!("First is 0 and last is {}", y),
        (x, 0) => println!("First is {} and last is 0", x),
        (x, y) => println!("{} and {}", x, y),
    }
}