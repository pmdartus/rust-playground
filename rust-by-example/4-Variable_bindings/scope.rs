fn main() {
    let outer = 1;

    {
        let inner = 2;
        println!("In block inner={}", inner);

        let outer = 3;
        println!("In block outer={}", outer);
    }

    println!("In main outer={}", outer);

    let outer = 5;

    println!("In main outer={}", outer);
}