#[derive(Debug)]
struct Structure(i32);

#[derive(Debug)]
struct Deep(Structure);

#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8
}

fn main() {
    println!("Structure {:?}", Structure(3));
    println!("Structure pretty {:#?}", Structure(3));

    println!("Deep {:?}", Deep(Structure(5)));
    println!("Deep pretty {:#?}", Deep(Structure(5)));

    let name = "Peter";
    let age = 10;
    let peter = Person { name, age };
    println!("Person {:?}", peter);
    println!("Person pretty {:#?}", peter);
}