use std::io::prelude::*;
use std::io::BufReader;
use std::net::{TcpListener, TcpStream};

fn main() {
    run();
}

fn run() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap_or_else(|e| {
        panic!("Something went wrong when binding the socket: {}", e);
    });

    for incoming in listener.incoming() {
        match incoming {
            Ok(stream) => {
                println!("New connection {:?}", stream);
                handle_connection(stream);
            }
            Err(e) => {
                panic!("Something went wrong in the connection: {}", e);
            }
        }
    }
}

fn handle_connection(stream: TcpStream) {
    let mut reader = BufReader::new(stream);
    
    let mut request_line = String::new();
    reader.read_line(&mut request_line);

    println!("Request line: {}", request_line);

    let mut headers: Vec<String> = vec![];
    loop {
        let mut next_line = String::new();
        reader.read_line(&mut next_line);

        if next_line == "\r\n" {
            break;
        } else {
            headers.push(next_line);
        }
    }

    println!("headers:\n{:#?}", headers);
}

// fn read_next_line(&mut reader: BufReader<TcpStream>) -> String {
//     let mut line = String::new();
//     reader.read_line(&mut line);

//     line.pop().pop()
// }