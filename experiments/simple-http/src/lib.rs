use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader, BufWriter};
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::path::Path;
use std::time::Instant;

mod header;
mod request;
mod response;
mod status;

use request::Request;
use response::Response;

#[derive(Debug)]
pub struct Config {
    pub port: u16,
    pub path: String,
}

pub fn run(config: Config) -> Result<(), Box<Error>> {
    let server = Server::new(config);
    server.start()
}

struct Server {
    config: Config,
}

impl Server {
    fn new(config: Config) -> Server {
        Server { config }
    }

    fn start(&self) -> Result<(), Box<Error>> {
        let addr = SocketAddr::from(([127, 0, 0, 1], self.config.port));
        let listener = TcpListener::bind(addr)?;

        println!("Server running at: http://{}", addr);

        for incoming in listener.incoming() {
            match incoming {
                Ok(stream) => {
                    self.handle_connection(stream);
                }
                Err(e) => {
                    // TODO: Improve error handling
                    panic!("Something went wrong in the connection: {}", e);
                }
            }
        }

        Ok(())
    }

    fn handle_connection(&self, stream: TcpStream) {
        let mut reader = BufReader::new(&stream);
        let mut writer = BufWriter::new(&stream);

        let start_time = Instant::now();

        let request = request::Request::read(&mut reader).unwrap();

        let response = self.handle_request(&request);
        let response_byte = response.serialize().into_bytes();

        writer.write(&response_byte);

        let duration = start_time.elapsed();

        println!(
            "{method} {uri} {status} - {response_time:?}",
            method = request.method,
            uri = request.uri,
            status = response.status.code,
            response_time = duration
        );
    }

    fn handle_request(&self, request: &Request) -> Response {
        match self.resolve_path(&request) {
            Some(path) => {
                let mut f = File::open(path).unwrap();

                let mut contents = String::new();
                f.read_to_string(&mut contents);

                let mut response = Response::new(request.http_version.to_string(), status::OK);
                response.body = Some(contents);

                response
            }
            None => Response::new(request.http_version.to_string(), status::NOT_FOUND),
        }
    }

    fn resolve_path(&self, request: &Request) -> Option<String> {
        let relative_uri = request.uri[1..].to_owned();
        let requested_path = Path::new(&self.config.path).join(&relative_uri);

        if requested_path.exists() {
            return Some(requested_path.to_str().unwrap().to_owned());
        } else if requested_path.is_dir() {
            let requested_path_index = requested_path.join("index.html");

            if requested_path_index.exists() {
                return Some(requested_path_index.to_str().unwrap().to_owned());
            }
        } else if None == requested_path.extension() {
            let requested_path_html = requested_path.with_extension("html");

            if requested_path_html.exists() {
                return Some(requested_path_html.to_str().unwrap().to_owned());
            }
        }

        None
    }
}
