fn create_fn() -> Box<Fn()> {
    let text = "Fn";

    let closure = Box::new(move || println!("In closure {}", text));

    println!("Out closure {}", text);

    closure
}

fn main() {
    let fn_plain = create_fn();

    fn_plain();
}