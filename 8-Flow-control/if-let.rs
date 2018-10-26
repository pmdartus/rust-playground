#[derive(Debug)]
enum Foo {
    Bar,
    Baz(u32),
}

fn main() {
    let number = Some(7);
    let letter: Option<i32> = None;
    
    if let Some(i) = number {
        println!("Match {:?}", i);
    }

    if let Some(i) = letter {
        println!("Match {:?}", i);
    } else {
        println!("Not matched {:?}", letter);
    }

    let a = Foo::Bar;
    let b = Foo::Baz(100);

    if let Foo::Bar = a {
        println!("a is a bar {:?}", a);
    }

    if let Foo::Baz(val) = b {
        println!("b is a baz {:?}", val);
    }
}