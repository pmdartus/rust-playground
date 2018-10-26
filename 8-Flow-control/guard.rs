fn main() {
    let pair = (1, -1);

    match pair {
        (x, y) if x == y => println!("Twin"),
        (x, y) if x + y == 0 => println!("0 sum"),
        _ => println!("Other")
    }
}