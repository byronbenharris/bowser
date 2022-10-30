use std::env;
use std::io::{Write, Read};
use std::net::{TcpStream, Shutdown};
use std::collections::HashMap;

// cargo run http://example.com:80/index.html

fn request(url:&String) -> (HashMap<String, String>, String) {
    // parse url into host and path
    assert!(url.starts_with("http://"));
    let trimmed = url.strip_prefix("http://").expect("Unable to trim url!");
    let mut split = trimmed.split('/');
    let host = split.next().expect("Unable to find host");
    let path = format!("/{}", split.next().expect("Unable to find path"));
    println!("HOST: {}", host);
    println!("PATH: {}", path);

    // open the TCP socket
    let mut stream = TcpStream::connect(host).expect("Failed to create socket!");
    println!("Opened Socket");

    // prepare HTTP request
    let mut http_req = String::new();
    http_req.push_str(format!("GET {} HTTP/1.1\r\n", path).as_str());
    http_req.push_str(format!("Host: {}\r\n\r\n", host).as_str());

    println!("{}", http_req);

    // send HTTP request as bytes via TCP stream
    let req_bytes: &[u8] = http_req.as_bytes();
    stream.write_all(req_bytes).expect("Failed to writes bytes for request!");
    println!("Sent Request");

    // read the response via TCP stream
    let mut http_res = String::new();
    stream.read_to_string(&mut http_res).expect("Failed to read response!");
    println!("Got Response");
    println!("{}", http_res);

    // idk if we need following lines: socket seems to auto-close
    // // close the TCP connection
    // stream.shutdown(Shutdown::Both).expect("Failed to shutdown socket!");
    // println!("Closed Socket");

    // parse the response into headers and body
    let mut lines = http_res.lines();

    let status_line = lines.next().expect("No lines");
    let mut status_line_split = status_line.split_whitespace();
    status_line_split.next();
    assert_eq!(Some("200"), status_line_split.next());

    let mut headers = HashMap::new();
    loop {
        let line = lines.next();
        match line {
            None => { break; },
            Some(inner) => {
                if inner == "" { break; }
                let mut split_line = inner.splitn(2, ":");
                let header = split_line.next().unwrap();
                let value = split_line.next().unwrap();
                headers.insert(header.to_lowercase(), value.trim().to_lowercase());
                continue;
            }
            other => { continue; }
        }
    }

    println!("HEADERS: {:#?}", headers);

    assert!(!headers.contains_key("transfer-encoding"));
    assert!(!headers.contains_key("content-encoding"));

    let mut body = String::new();
    loop {
        let line = lines.next();
        
        match line {
            None => { break; },
            Some(inner) => {
                body.push_str(inner);
                continue;
            }
            other => { continue; }
        }
    }

    println!("BODY: {}", body);

    (headers, body)
}

fn load(url:&String) {
    let (headers, body) = request(url);
    show(body);
}

fn show(body:String) {
    let mut in_angle = false;
    for c in body.chars() {
        if c == '<' {
            in_angle = true;
        } else if c == '>' {
            in_angle = false;
        } else if !in_angle {
            print!("{}", c);
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let url = &args[1];
    load(url);
}
