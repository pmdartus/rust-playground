use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::net::TcpStream;

#[derive(Debug)]
pub struct Request {
    pub method: String,
    pub uri: String,
    pub http_version: String,
    pub headers: Vec<Header>,
}

#[derive(Debug)]
pub struct Header {
    pub name: String,
    pub content: String,
}

impl Request {
    // TODO: Improve error handling
    // TODO: Modularize
    pub fn read(reader: &mut BufReader<&TcpStream>) -> io::Result<Request> {
        let mut request_line = String::new();
        reader.read_line(&mut request_line)?;

        let request_line_items: Vec<&str> = request_line.split(' ').collect();

        let method = request_line_items[0].to_owned();
        let uri = request_line_items[1].to_owned();

        // Remove the "\r\n"
        let http_version = request_line_items[2][..(request_line_items[2].len() - 2)].to_owned();

        let mut headers: Vec<Header> = vec![];
        loop {
            let mut next_line = String::new();
            reader.read_line(&mut next_line)?;

            if next_line == "\r\n" {
                break;
            } else {
                let column_index = next_line.find(':').unwrap();
                let (name, content) = next_line.split_at(column_index);

                headers.push(Header {
                    name: name.to_owned(),
                    // Remove the ": " and "\r\n" prefixing the content
                    content: content[2..(content.len() - 2)].to_owned(),
                });
            }
        }

        Ok(Request {
            method,
            uri,
            http_version,
            headers,
        })
    }
}
