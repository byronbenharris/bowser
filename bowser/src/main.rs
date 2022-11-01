use druid::widget::{Flex, Label, Scroll};
use druid::{AppLauncher, LocalizedString, Widget, WindowDesc};
use std::collections::HashMap;
use std::io::{BufRead, BufReader, Read, Write};
use std::net::TcpStream;
use std::str;
use std::{env, vec};

// cargo run http://example.com:80/index.html

fn request(url: &String) -> (HashMap<String, String>, String) {
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
    let body = str::from_utf8(&body)
        .expect("error converting [u8] to string")
        .to_string();

    (headers, body)
}

fn load(url: &String) -> impl Widget<()> {
    let (_headers, body) = request(url);
    let tokens = lex(body);
    let body_widget = layout(tokens);
    let mut col = Flex::column();
    col.add_child(body_widget);
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
    body
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
    out
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let url = &args[1];
    AppLauncher::with_window(
        WindowDesc::new(load(url))
            .title(LocalizedString::new("Bowser Title").with_placeholder("Bowser")),
    )
    .launch(())
    .expect("failed to launch gui");
}
