use std::io::prelude::*;
use std::io::{BufReader, BufWriter};
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

#[derive(Debug)]
struct Request {
    method: String,
    uri: String,
    http_version: String,
    headers: Vec<Header>,
}

#[derive(Debug)]
struct Header {
    name: String,
    content: String,
}

fn handle_connection(mut stream: TcpStream) {
    let mut reader = BufReader::new(&stream);
    let mut writer = BufWriter::new(&stream);

    let mut request_line = String::new();

    reader.read_line(&mut request_line);
    let (method, uri, http_version) = parse_request_line(&request_line);

    let mut headers: Vec<Header> = vec![];
    loop {
        let mut next_line = String::new();
        reader.read_line(&mut next_line);

        if next_line == "\r\n" {
            break;
        } else {
            let header = parse_header(&next_line);

            headers.push(header);
        }
    }

    let request = Request {
        method,
        uri,
        http_version,
        headers,
    };

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

fn parse_request_line(line: &String) -> (String, String, String) {
    let items: Vec<&str> = line.split(' ').collect();

    (
        items[0].to_owned(),
        items[1].to_owned(),
        // Remove the "\r\n"
        items[2][..(items[2].len() - 2)].to_owned(),
    )
}

// TODO: Add better error handling
fn parse_header(line: &String) -> Header {
    let column_index = line.find(':').unwrap();
    let (name, content) = line.split_at(column_index);

    Header {
        name: name.to_owned(),
        // Remove the ": " and "\r\n" prefixing the content
        content: content[2..(content.len() - 2)].to_owned(),
    }
}
