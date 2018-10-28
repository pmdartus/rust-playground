use std::error::Error;
use std::io::prelude::*;
use std::io::{BufReader, BufWriter};
use std::net::{TcpListener, TcpStream, SocketAddr};

mod request;

#[derive(Debug)]
pub struct Config {
    pub port: u16,
    pub path: String,
}

pub fn run(config: Config) -> Result<(), Box<Error>> {
    let addr = SocketAddr::from(([127, 0, 0, 1], config.port));
    let listener = TcpListener::bind(addr)?;

    println!("Server running at: http://{}", addr);

    for incoming in listener.incoming() {
        match incoming {
            Ok(stream) => {
                println!("New connection {:?}", stream);
                handle_connection(stream);
            }
            Err(e) => {
                // TODO: Improve error handling
                panic!("Something went wrong in the connection: {}", e);
            }
        }
    }

    Ok(())
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
        "\r\n",
    ]
        .join("\r\n")
        .to_string();
    let response = headers + &body;

    println!("Response:\n{:#?}", response);

    writer.write(&response.into_bytes());
}
