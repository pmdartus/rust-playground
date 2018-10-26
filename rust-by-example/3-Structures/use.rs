enum Status {
    Poor,
    Middle,
    Rich,
}

fn main() {
    use Status::{Poor, Rich};

    let status = Poor;

    match status {
        Poor => println!("POOR!"),
        Rich => println!("RICH!"),
        Status::Middle => println!("MIDDLE"),
    }
}