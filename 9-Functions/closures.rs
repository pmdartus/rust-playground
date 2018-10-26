fn main() {
    let increment = |i: i32| -> i32 { i + 1 };
    let increment_infered = |i| i + 1;

    println!("{}", increment(1));
    println!("{}", increment_infered(1));
}