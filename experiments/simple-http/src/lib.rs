use std::io::prelude::*;
use std::io::{BufReader, BufWriter};
use std::net::{TcpListener, TcpStream};

mod request;

pub fn run() {
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

fn handle_connection(mut stream: TcpStream) {
    let mut reader = BufReader::new(&stream);
    let mut writer = BufWriter::new(&stream);

    let request = request::Request::read(&mut reader).unwrap();
    
    println!("Incoming request:\n{:#?}", request);

    let body = format!("echo: {}", request.uri);
    let headers = [
        "HTTP/1.1 200 OK", 
        &format!("Cotent-Length: {}", body.bytes().len()),
        "\r\n"
    ].join("\r\n").to_string();
    let response = headers + &body;

    println!("Response:\n{:#?}", response);
    
    writer.write(&response.into_bytes());
}
