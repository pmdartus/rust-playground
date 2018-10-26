fn age() -> u32 {
    2
}

fn main() {
    match age() {
        0 => println!("0"),
        n @ 1...12 => println!("Youth {}", n),
        n @ 13...19 => println!("Teen {}", n),
        n => println!("Older {}", n),
    }
}