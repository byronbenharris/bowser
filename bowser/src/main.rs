use druid::piet::InterpolationMode;
use druid::widget::{FillStrat, Flex, Image, Label, Scroll};
use druid::{AppLauncher, ImageBuf, LocalizedString, Widget, WindowDesc};
use std::collections::HashMap;
use std::io::{BufRead, BufReader, Read, Write};
use std::net::TcpStream;
use std::{env, vec};
use std::{fs, str};

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

    let mut headers = HashMap::new();
    loop {
        let mut line = String::new();
        reader.read_line(&mut line).expect("error reading header");
        match line.as_str() {
            "\r\n" => {
                break;
            }
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

    let length = match headers.get("content-length") {
        None => {
            println!("no body content!");
            0
        }
        Some(x) => x.parse::<usize>().expect("error parsing content length"),
    };
    let mut body = vec![0u8; length];
    reader.read_exact(&mut body).expect("error reading body");
    return (headers, body);
}

fn request(url: &String) -> (HashMap<String, String>, Vec<u8>) {

    let mut scheme_split = url.splitn(2, "://");
    let scheme = scheme_split.next().expect("No scheme provided in url!");
    let content = scheme_split.next().expect("No content provided in url!");

    match scheme {
        "http" => { return request_web(content, false); },
        "https" => { return request_web(content, true); },
        "file" => { return request_file(content); },
        "view-source" => { panic!("Scheme 'view-source' is not yet implemented!");  }, // TODO
        "data" => { panic!("Scheme 'data' is not yet implemented!"); }, // TODO
        _ => { panic!("Unknown scheme provided in request url!"); }
    }
}

fn load(url: &String) -> impl Widget<()> {
    let (headers, body) = request(url);
    assert!(headers.contains_key("content-type"));
    let content_type = headers
        .get("content-type")
        .unwrap()
        .splitn(2, ';')
        .next()
        .unwrap();
    let mut col = Flex::column();
    match content_type {
        "text/html" => {
            let body_str = str::from_utf8(&body).expect("Failed to convert [u8] to string");
            let tokens = lex(body_str.to_string());
            let body_widget = layout(tokens);
            col.add_child(body_widget);
        }
        "image/png" => {
            let img_data =
                ImageBuf::from_data(&body).expect("Failed to store bytes in image buffer");
            let img = Image::new(img_data)
                .fill_mode(FillStrat::Fill)
                .interpolation_mode(InterpolationMode::Bilinear);
            col.add_child(img);
        }
        _ => col.add_child(Label::new("Unknown content type")),
    }
    Scroll::new(col).vertical()
}

enum Token {
    Text(String),
    Tag(String),
}

fn layout(tokens: Vec<Token>) -> impl Widget<()> {
    let mut body_text = String::new();
    for tok in tokens {
        if let Token::Text(text) = tok {
            body_text.push_str(&text);
        }
    }
    let mut body = Label::new(body_text);
    body.set_line_break_mode(druid::widget::LineBreaking::WordWrap);
    return body;
}

fn lex(body: String) -> Vec<Token> {
    let mut out = Vec::new();
    let mut in_tag = false;
    let mut text = String::new();
    for c in body.chars() {
        if c == '<' {
            in_tag = true;
            if !text.is_empty() {
                out.push(Token::Text(text));
                text = String::new();
            }
        } else if c == '>' {
            in_tag = false;
            out.push(Token::Tag(text));
            text = String::new();
        } else {
            text.push(c);
        }
    }
    if !in_tag && !text.is_empty() {
        out.push(Token::Text(text));
    }
    return out;
}

fn main() {

    let args: Vec<String> = env::args().collect();
    let url = &args[1];

    AppLauncher::with_window(
        WindowDesc::new(load(url))
            .title(LocalizedString::new("Bowser Title")
            .with_placeholder("Bowser")),
    )
    .launch(())
    .expect("failed to launch gui");
}
