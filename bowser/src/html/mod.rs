
use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
use std::vec;

const VOID_TAGS: [&str; 14] = [
    "area", "base", "br", "col", "embed", "hr", "img", "input",
    "link", "meta", "param", "source", "track", "wbr"
];

#[derive(Debug)]
pub struct DOMNode {
    pub data: Data,
    pub children: RefCell<Vec<Rc<RefCell<DOMNode>>>>
}

impl DOMNode {
    fn new(data: Data) -> DOMNode {
        return DOMNode { data, children: RefCell::new(vec!()) };
    }

    fn add_child(&mut self, child: &Rc<RefCell<DOMNode>>) {
        self.children.borrow_mut().push(Rc::clone(&child));
    }
}

#[derive(Debug)]
pub enum Data {
    Text(Text),
    Element(Element),
}

#[derive(Debug)]
pub struct Element {
    pub tag: String,
    pub attributes: HashMap<String, String>,
}

impl Element {
    fn new(tag: String) -> Option<Element> {
        if tag.starts_with("!") { return None; }
        let (tag, attributes) = get_attributes(tag);
        return Some(Element { tag, attributes });
    }
}

#[derive(Debug)]
pub struct Text {
    pub text: String,
}

impl Text {
    fn new(text: String) -> Option<Text> {
        if text.trim().is_empty() { return None; }
        return Some(Text { text });
    }
}

fn get_attributes(text: String) -> (String, HashMap<String, String>) {
    
    let mut parts = text.split(char::is_whitespace);
    let tag = parts.next().unwrap().to_lowercase();
    
    let mut attributes = HashMap::new();
    while let Some(attr_pair) = parts.next() {
        if attr_pair.contains("=") {
            
            let mut attr_split = attr_pair.splitn(1, "=");
            let key = attr_split.next().unwrap().to_string();
            let mut value = attr_split.next().unwrap_or("").to_string();
            
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

pub fn parse(body: &String) -> Rc<RefCell<DOMNode>> {

    let root = DOMNode::new(Data::Element(Element::new(String::from("bowser")).unwrap()));
    let root_ptr = Rc::new(RefCell::new(root));
    let mut parent_stack:Vec<Rc<RefCell<DOMNode>>> = Vec::new();
    parent_stack.push(Rc::clone(&root_ptr));

    let mut in_tag = false;
    let mut inner_text = String::new();
    for c in body.chars() {
        if c == '<' {
            if let Some(text) = Text::new(inner_text) {
                let node = DOMNode::new(Data::Text(text));
                parent_stack.last().unwrap().borrow_mut().add_child(&Rc::new(RefCell::new(node)));
            }
            in_tag = true;
            inner_text = String::new();
        } else if c == '>' {
            if !inner_text.starts_with("/") {
                if let Some(elem) = Element::new(inner_text) {
                    let not_void = !VOID_TAGS.contains(&elem.tag.as_str());
                    let node = DOMNode::new(Data::Element(elem));
                    let node_ptr = Rc::new(RefCell::new(node));
                    let parent = parent_stack.last().unwrap();
                    parent.borrow_mut().add_child(&Rc::clone(&node_ptr));
                    if not_void {
                        parent_stack.push(Rc::clone(&node_ptr));
                    }
                }
            } else {
                {
                    let parent = parent_stack.last().unwrap().borrow();
                    if let Some(open_tag) = match &parent.data {
                        Data::Text(_) => None,
                        Data::Element(elem) => Some(elem.tag.as_str()),
                    } {
                        let close_tag = inner_text.split(' ').next().unwrap().get(1..).unwrap();
                        if open_tag != close_tag {
                            panic!("Invalid closing tag in parsing: {} (open) != {} (close)", 
                                open_tag, close_tag);
                        }
                    }
                }         
                parent_stack.pop();
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
        let node = DOMNode::new(Data::Text(text));
        parent_stack.last().unwrap().borrow_mut().add_child(&Rc::new(RefCell::new(node)));
    }

    return root_ptr;
}

fn print_indent(indent: i32) {
    for _ in 0..indent { print!(" "); }
}

pub fn print_dom(node: &Rc<RefCell<DOMNode>>, indent: i32) {
    print_indent(indent);
    match &node.borrow().data {
        Data::Text(text) => { 
            println!("{}", text.text);
        },
        Data::Element(elem) => {
            let not_void = !VOID_TAGS.contains(&elem.tag.as_str());
            println!("<{}>", elem.tag);
            for child in &*node.borrow().children.borrow() {
                print_dom(child, indent + 2);
            }
            if not_void {
                print_indent(indent);
                println!("</{}>", elem.tag);
            }
        }
    }
}