fn main() {
    let mut optional = Some(0);

    while let Some(i) = optional {
        if i > 9 {
            println!("I am done");
            optional = None;
        } else {
            println!("Current: {:?}", optional);
            optional = Some(i + 1);
        }
    }
}