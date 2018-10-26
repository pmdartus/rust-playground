pub mod nested;

pub fn function() {
    println!("called `my::function()`");
}

fn private() {
    println!("called `my::private()`");
}

pub fn indirect() {
    println!("called `my::indirect()`");
    private();
}