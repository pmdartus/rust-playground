use header::Header;
use status::Status;

pub struct Response {
    pub http_version: String,
    pub status: Status,
    pub headers: Vec<Header>,
    pub body: Option<String>,
}

impl Response {
    pub fn new(http_version: String, status: Status) -> Response {
        Response {
            http_version,
            status,
            headers: Vec::new(),
            body: None,
        }
    }

    pub fn serialize(&self) -> String {
        let mut response = vec![];

        let response_line = format!(
            "{} {} {}",
            self.http_version, self.status.code, self.status.reason
        );
        response.push(response_line);

        let mut headers: Vec<String> = self
            .headers
            .iter()
            .map(|x| format!("{}: {}", x.name, x.content))
            .collect();
        response.append(&mut headers);

        if let Some(body) = &self.body {
            response.push(body.to_string());
        }

        response.join("\r\n").to_string()
    }
}
