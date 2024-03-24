pub enum Header {
    SUCCESS(usize),
    NOTFOUND
}

pub struct Response {
    headers: String,
    contents: String
}

impl Response {
    pub fn new(headers: String, contents: String) -> Self {
        Response {
            headers,
            contents
        }
    }

    // This encodes all its information in a single String.
    pub fn normalized(&self) -> String {
        let mut response = String::new();
        response.push_str(self.headers.as_str());
        response.push_str(self.contents.as_str());

        response
    }
}

pub fn get_request_path(http_request: &Vec<String>) -> String {
    let request_line: Vec<&str> = http_request.get(0).unwrap().split(" ").collect();
    let req_path = request_line.get(1).unwrap();

    println!("{}", req_path);
    String::from(req_path.to_owned())
}

pub fn create_headers(header: Header) -> String {
    match header {
        Header::SUCCESS(length) => String::from(format!("HTTP/1.1 200 OK\r\nX-Powered-By: XWaiter\r\nContent-Length: {length}\r\n\r\n")),
        Header::NOTFOUND => String::from("HTTP/1.1 404 OK\r\nX-Powered-By: XWaiter\r\n\r\n")
    }
}