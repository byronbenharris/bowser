use druid::piet::InterpolationMode;
use druid::widget::{
    FillStrat, 
    Flex, 
    Image, 
    Label, 
    Scroll
};

use druid::{
    AppLauncher,
    FontDescriptor, 
    FontFamily,
    FontWeight, 
    FontStyle, 
    ImageBuf, 
    LocalizedString, 
    Widget, 
    WindowDesc
};

use std::cell::RefCell;
use std::collections::HashMap;
use std::io::{BufRead, BufReader, Read, Write};
use std::net::TcpStream;
use std::{env, vec};
use std::{fs, str};
use std::rc::Rc;

#[derive(Debug)]
struct Node {
    data: Data,
    children: RefCell<Vec<Rc<Node>>>
}

impl Node {
    fn new(data: Data) -> Node {
        return Node {data, children: RefCell::new(vec!()) };
    }

    fn add_child(&mut self, child: &Rc<Node>) {
        self.children.borrow_mut().push(Rc::clone(&child));
    }
}

#[derive(Debug)]
enum Data {
    Text(Text),
    Element(Element),
}

#[derive(Debug)]
struct Element {
    tag: String,
    attributes: HashMap<String, String>,
    
}

impl Element {
    fn new(tag: String) -> Option<Element> {
        if tag.starts_with("!") { return None; }
        let (tag, attributes) = get_attributes(tag);
        return Some(Element { tag, attributes });
    }
}

#[derive(Debug)]
struct Text {
    text: String,
}

impl Text {
    fn new(text: String) -> Option<Text> {
        if text.trim().is_empty() { return None; }
        return Some(Text { text });
    }
}

#[derive(Debug)]
struct Style {
    size: f64, 
    bold: bool, 
    italic: bool,
}

const VOID_TAGS: [&str; 14] = [
    "area", "base", "br", "col", "embed", "hr", "img", "input",
    "link", "meta", "param", "source", "track", "wbr"
];

const BLOCK_ELEMENTS: [&str; 37] = [
    "html", "body", "article", "section", "nav", "aside",
    "h1", "h2", "h3", "h4", "h5", "h6", "hgroup", "header",
    "footer", "address", "p", "hr", "pre", "blockquote",
    "ol", "ul", "menu", "li", "dl", "dt", "dd", "figure",
    "figcaption", "main", "div", "table", "form", "fieldset",
    "legend", "details", "summary"
];

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
    panic!("Scheme 'view-source' is not yet implemented!");
}

fn request(url: &String) -> (HashMap<String, String>, Vec<u8>) {

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
            let root = parse(&body_str.to_string());
            print_tree(Rc::clone(&root), 0);
            let body_widgets = recurse(
                &Rc::clone(&root), &Style { size: 16.0, bold: false, italic: false });
            for widget in body_widgets {
                col.add_child(widget);
            }
        },
        "image/png" => {
            let img_data =
                ImageBuf::from_data(&body).expect("Failed to store bytes in image buffer");
            let img = Image::new(img_data)
                .fill_mode(FillStrat::Fill)
                .interpolation_mode(InterpolationMode::Bilinear);
            col.add_child(img);
        },
        _ => col.add_child(Label::new("Unknown content type")),
    }
    Scroll::new(col).vertical()
}

fn get_font(style: &Style) -> FontDescriptor {
    return FontDescriptor::new(FontFamily::SERIF)
        .with_size(style.size)
        .with_weight(match style.bold {
            true => FontWeight::BOLD,
            false => FontWeight::REGULAR,
        })
        .with_style(match style.italic {
            true => FontStyle::Italic,
            false => FontStyle::Regular,
        });
}

fn open_tag(tag: &String, style: &Style) -> Style {
    match tag.as_str() {
        "b" => { return Style { bold: true, ..(*style) } },
        "i" => { return Style { italic: true, ..(*style) } },
        "bigger" => { return Style { size: style.size + 2.0, ..(*style) } },
        "smaller" => { return Style { size: style.size - 2.0, ..(*style) } },
        _ => { return Style{ ..(*style) }; }
    }
}

fn label(text: &String, style: &Style) -> impl Widget<()> {
    let font = get_font(style);
    let mut label = Label::new(text.as_str()).with_font(font);
    label.set_line_break_mode(druid::widget::LineBreaking::WordWrap);
    return label;
}

fn recurse(node: &Rc<Node>, style: &Style) -> Vec<impl Widget<()>> {
    let mut body = Vec::new();
    for child in &*node.children.borrow() {
        for label in recurse(&Rc::clone(&child), &style) {
            body.push(label);
        }
    }
    return body;

    // match &node.borrow().data {
    //     Data::Text(text) => {
    //         let mut body = Vec::new();
    //         body.push(label(&text.text, style)); 
    //         return body;
    //     },
    //     Data::Element(elem) => { 
    //         let style = open_tag(&elem.tag, style);
    //         let mut body = Vec::new();
    //         for child in (*node.borrow()).children {
    //             for label in recurse(Rc::clone(child), &style) {
    //                 body.push(label);
    //             }
    //         }
    //         return body;
    //     }
    // }
}

fn get_attributes(text: String) -> (String, HashMap<String, String>) {
    
    let mut parts = text.split(char::is_whitespace);
    let tag = parts.next().unwrap().to_lowercase();
    
    let mut attributes = HashMap::new();
    while let Some(attr_pair) = parts.next() {
        if attr_pair.contains("=") {
            
            let mut attr_split = attr_pair.splitn(1, "=");
            let key = attr_split.next().unwrap().to_string();
            let mut value = attr_split.next().unwrap().to_string();
            
            if value.len() > 2 && 
                (value.starts_with("'") || value.starts_with("\"")) {
                    value = value[1..value.len()-1].to_owned();
            }
            
            attributes.insert(key.to_lowercase(), value);
        
        } else {
            attributes.insert(attr_pair.to_lowercase(), String::new());
        }
    }

    return (tag, attributes);
}

fn print_tree(node: Rc<RefCell<Node>>, indent: i32) {

    for _ in 0..indent { 
        print!(" "); 
    }

    match &node.borrow().data {
        Data::Text(text) => { 
            println!("{}", text.text);
        },
        Data::Element(elem) => { 
            println!("<{}>", elem.tag); 
            for child in *node.borrow().children.borrow() {
            }
            // for child in elem.children {
            //     print_tree(child, indent + 2);
            // }
            println!("</{}>", elem.tag); 
        }
    }
}

fn parse(body: &String) -> Rc<RefCell<Node>> {

    let root = Node::new(Data::Element(Element::new(String::from("root")).unwrap()));
    let root_ptr = Rc::new(RefCell::new(root));
    let mut parent_queue:Vec<Rc<RefCell<Node>>> = Vec::new();
    parent_queue.push(Rc::clone(&root_ptr));

    let mut in_tag = false;
    let mut inner_text = String::new();
    for c in body.chars() {
        if c == '<' {
            if let Some(text) = Text::new(inner_text) {
                let node = Node::new(Data::Text(text));
                parent_queue.last().unwrap().borrow_mut().add_child(&Rc::new(node));
            }
            in_tag = true;
            inner_text = String::new();
        } else if c == '>' {
            if !inner_text.starts_with("/") {
                if let Some(elem) = Element::new(inner_text) {
                    let node = Node::new(Data::Element(elem));
                    let parent = parent_queue.last().unwrap();
                    parent.borrow_mut().add_child(&Rc::new(node));
                    // TODO!!
                    // if !VOID_TAGS.contains(&elem.tag.as_str()) {
                    //     parent_queue.push(Rc::new(RefCell::new(node)));
                    // }
                }
            } else {
                // TODO!!
                // {
                //     let parent = parent_queue.last().unwrap().borrow();
                //     if let Some(open_tag) = match parent.data {
                //         Data::Text(_) => None,
                //         Data::Element(elem) => Some(elem.tag),
                //     } {
                //         let close_tag = inner_text.split(' ').next().unwrap().get(1..).unwrap();
                //         if open_tag == close_tag {
                //             panic!("Invalid closing tag in parsing: {} (open) != {} (close)", 
                //                 open_tag, close_tag);
                //         }
                //     }                    
                // }
                parent_queue.pop();
            }
            in_tag = false;
            inner_text = String::new();
        } else {
            inner_text.push(c);
        }
    }

    if in_tag {
        panic!("Invalid HTML: EOF in tag!")
    }

    if let Some(text) = Text::new(inner_text) {
        let node = Node::new(Data::Text(text));
        parent_queue.last().unwrap().borrow_mut().add_child(&Rc::new(node));
    }

    return root_ptr;
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
