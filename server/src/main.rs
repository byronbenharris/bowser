use std::net::{TcpListener, TcpStream};
use std::collections::HashMap;

static ENTRIES: Vec<String> = [ "Ben was here".to_string() ].to_vec();

fn not_found(url: String, method: String) -> String {
    return format!("<!doctype html><h1>{} {} not found!</h1>", method, url);
}

fn add_entry(params: HashMap<String, String>) -> String {
    if params.contains_key("guest") {
        ENTRIES.push(params.get("guest"));
    }
    return show_comments();
}

fn form_decode(body: Vec<u8>) -> HashMap<String, String> {
    
    let params = HashMap::new();
    let mut and_split = body.split("&");
    
    while let mut field = and_split.next().unwrap() {
        let mut equ_split = field.splitn(2, "=");
        let name = split_line.next().unwrap();
        let value = split_line.next().unwrap();
        // name = urllib.parse.unquote_plus(name)
        // value = urllib.parse.unquote_plus(value)
        params.add(name, value);
    }

    return params;
}

fn do_request(
    method: String, 
    url: String, 
    headers: HashMap<String, String>, 
    body: Vec<u8>
) -> (String, String) {
    if method == "GET" && url == "/" {
        return ("200 OK", show_comments());
    } else if method == "POST" && url == "/add" {
        return ("200 OK", add_entry(form_decode(body)));
    } else {
        return ("404 Not Found", not_found(url, method));
    }
}

fn show_comments() -> String {
    let out = "<!doctype html>";
    for entry in ENTRIES {
        out += "<p>" + entry + "</p>";
    }
    out += "<form action=add method=post>";
    out += "<p><input name=guest></p>";
    out += "<p><button>Sign the book!</button></p>";
    out += "</form>";
    return out;
}

fn handle_connection(stream: TcpStream) {

    // req = conx.makefile("b")
    // reqline = req.readline().decode('utf8')
    // method, url, version = reqline.split(" ", 2)
    // assert method in ["GET", "POST"]

    // // read the response via TCP stream
    // let mut reader = BufReader::new(&stream);
    // println!("Waiting for response");
    // let mut status_line = String::new();
    // reader
    //     .read_line(&mut status_line)
    //     .expect("error reading status line");
    // let mut status_line_split = status_line.split_whitespace();
    // status_line_split.next();
    // assert_eq!(Some("200"), status_line_split.next());


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
        
    // let (status, body) = do_request(method, url, headers, body);
    // let response = format!("HTTP/1.0 {}\r\n", status);
    // // len(body.encode("utf8"))
    // let response += format!("Content-Length: {}\r\n", );
    // let response += format!("\r\n{}", body);
    // conx.send(response.encode('utf8'))

    // Send HTTP response as bytes via TCP stream
    let res_bytes: &[u8] = response.as_bytes();
    steam.write_all(res_bytes).expect("Failed to writes bytes for response!");
    println!("Sent Response");

    // Close the TCP stream
    stream.close();
    println!("Closed Stream");
}

fn listen(host: String, port: String) {

    let listener = TcpListener::bind(format!("{}:{}", host, port)).unwrap();

    loop {
        match listener.accept() {
            Ok((stream, addr)) => { 
                println!("new client: {addr:?}");
                handle_connection(stream);
            },
            Err(e) => println!("couldn't get client: {e:?}"),
        }
    }
}

fn main() {
    println!("Hello, world!");
    let host = "127.0.0.1";
    let port = "8080";
    listen(host, port);
}
