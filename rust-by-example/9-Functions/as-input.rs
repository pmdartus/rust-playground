fn apply<F>(mut f: F) where F: FnMut() {
    f()
}

fn apply_to_3<F>(f: F) -> i32 where F: Fn(i32) -> i32 {
    f(3)
}

fn main() {
    let name = "Bob".to_owned();

    let closure = || {
        println!("Hello {}", name);
    };

    apply(closure);
    apply(closure);

    let double = |x| 2 * x;
    println!("2 doubled {}", apply_to_3(double));
}