fn main() {
    let decimal = 65.4321_f32;

    let integer = decimal as u8;
    let character = integer as char;

    println!("Int {}, Char {}", integer, character);
}