enum WebEvent {
    PageLoad,
    Paste(String),
    Click { x: i64, y: i64 },
}

fn inspect(event: WebEvent) {
    match event {
        WebEvent::PageLoad => println!("Page loaded"),
        WebEvent::Click {x, y } => println!("Click x={} y={}", x, y),
        _ => println!("Other event"),
    }
}

fn main() {
    let load = WebEvent::PageLoad;
    let paste = WebEvent::Paste("hello".to_owned());
    let click = WebEvent::Click { x: 123, y: 456 };

    inspect(load);
    inspect(paste);
    inspect(click);
}