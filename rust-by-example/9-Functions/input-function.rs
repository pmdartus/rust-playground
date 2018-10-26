fn call_me<F>(f: F) where F: Fn() {
    f();
}

fn print_1() {
    println!("1");
}

fn main() {
    let print_2 = || println!("2");

    call_me(print_1);
    call_me(print_2);
}