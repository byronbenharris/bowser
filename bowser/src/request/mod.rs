
use std::collections::HashMap;
use std::io::{BufRead, BufReader, Read, Write};
use std::net::TcpStream;
use std::{fs, str, vec};

fn request_file(path: &str) -> (HashMap<String, String>, Vec<u8>) {
    let mut headers = HashMap::new();
    headers.insert("content-type".to_string(), "text/html".to_string());
    return (
        headers,
        fs::read(path).expect("Failed to read contents of file"),
    );
}

fn request_web(url: &str, secure: bool) -> (HashMap<String, String>, Vec<u8>) {

    let mut split = url.splitn(2, '/');
    let host = split.next().expect("Unable to find host");
    let path = format!("/{}", split.next().expect("Unable to find path"));
    let port  = if secure == false { 80 } else { 443 };

    println!("HOST: {}", host);
    println!("PATH: {}", path);

    // open the TCP socket
    let mut stream = TcpStream::connect(format!("{}:{}", host, port)).expect("Failed to create socket!");
    println!("Opened Socket");

    // prepare HTTP request
    let mut http_req = String::new();
    http_req.push_str(format!("GET {} HTTP/1.1\r\n", path).as_str());
    http_req.push_str(format!("Host: {}:{}\r\n", host, port).as_str());
    http_req.push_str("User-Agent: bowser-nom-nom-nom\r\n");
    http_req.push_str("Connection: close\r\n\r\n");
    
    println!("{}", http_req);

    // send HTTP request as bytes via TCP stream
    let req_bytes: &[u8] = http_req.as_bytes();
    stream
        .write_all(req_bytes)
        .expect("Failed to writes bytes for request!");
    println!("Sent Request");

    // read the response via TCP stream
    let mut reader = BufReader::new(&stream);
    println!("Waiting for response");
    let mut status_line = String::new();
    reader
        .read_line(&mut status_line)
        .expect("error reading status line");
    let mut status_line_split = status_line.split_whitespace();
    status_line_split.next();
    assert_eq!(Some("200"), status_line_split.next());

    // Read the headers
    let mut headers = HashMap::new();
    loop {
        let mut line = String::new();
        reader.read_line(&mut line).expect("error reading header");
        match line.as_str() {
            "\r\n" => { break; }
            data => {
                let mut split_line = data.splitn(2, ":");
                let header = split_line.next().unwrap();
                let value = split_line.next().unwrap();
                headers.insert(header.to_lowercase(), value.trim().to_lowercase());
            }
        }
    }

    println!("HEADERS: {:#?}", headers);
    assert!(!headers.contains_key("transfer-encoding"));
    assert!(!headers.contains_key("content-encoding"));

    // Read the body
    let length = match headers.get("content-length") {
        Some(x) => x.parse::<usize>().expect("error parsing content length"),
        None => {
            println!("no body content!");
            0
        }
    };

    let mut body = vec![0u8; length];
    reader.read_exact(&mut body).expect("error reading body");

    return (headers, body);
}

fn data(content: &str) -> (HashMap<String, String>, Vec<u8>) {
    let mut headers = HashMap::new();
    headers.insert("content-type".to_string(), "text/html".to_string());
    return (headers, content.as_bytes().to_vec());
}

fn view_source(_url: &str) -> (HashMap<String, String>, Vec<u8>) {
    todo!();
}

pub fn request(url: &String) -> (HashMap<String, String>, Vec<u8>) {

    let mut scheme_split = url.splitn(2, "://");
    let scheme = scheme_split.next().expect("No scheme provided in url!");
    let content = scheme_split.next().expect("No content provided in url!");

    match scheme {
        "data" => { return data(content); },
        "file" => { return request_file(content); },
        "http" => { return request_web(content, false); },
        "https" => { return request_web(content, true); },
        "view-source" => { return view_source(content); },
        _ => { panic!("Unknown scheme provided in request url!"); }
    }
}