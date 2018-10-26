use std::string;

struct Circle {
    radius: i32
}

impl string::ToString for Circle {
    fn to_string(&self) -> String {
        format!("[Radius {}]", self.radius)
    }
}

fn main() {
    let circle = Circle { radius: 12 };
    println!("{}", circle.to_string());

    let parsed: i32 = "5".parse().unwrap();
    let turbo_parsed = "32".parse::<i32>().unwrap();

    println!("{}", parsed + turbo_parsed);
}