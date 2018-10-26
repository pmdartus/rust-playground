fn main() {
    for i in 1..20 {
        println!("Iteration {}: {}", i,  fib(i));
    }
}

fn fib(i: u32) -> u32 {
    match i {
        0 | 1 => 1,
        _ => fib(i - 1) + fib(i - 2),
    }
}