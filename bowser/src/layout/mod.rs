use druid::widget::Label;
use druid::{FontDescriptor, FontFamily, FontWeight, FontStyle, Widget};
use std::cell::RefCell;
use std::str;
use std::rc::Rc;

use crate::html::{DOMNode, Data};

#[derive(Debug)]
pub struct LayoutNode {
    pub node: Rc<RefCell<DOMNode>>,
    pub inline: bool,
    pub children: RefCell<Vec<Rc<LayoutNode>>>,
}

impl LayoutNode {
    fn new(node: &Rc<RefCell<DOMNode>>, inline: bool) -> LayoutNode {
        return LayoutNode { node: Rc::clone(node), inline, children: RefCell::new(vec!()) };
    }

    fn add_child(&mut self, child: &Rc<LayoutNode>) {
        self.children.borrow_mut().push(Rc::clone(&child));
    }
}

#[derive(Debug)]
pub struct Style {
    size: f64, 
    bold: bool, 
    italic: bool,
}

impl Style {
    pub fn new() -> Style {
        return Style { size: 16.0, bold: false, italic: false };
    }
}

const BLOCK_ELEMENTS: [&str; 37] = [
    "html", "body", "article", "section", "nav", "aside",
    "h1", "h2", "h3", "h4", "h5", "h6", "hgroup", "header",
    "footer", "address", "p", "hr", "pre", "blockquote",
    "ol", "ul", "menu", "li", "dl", "dt", "dd", "figure",
    "figcaption", "main", "div", "table", "form", "fieldset",
    "legend", "details", "summary"
];

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

pub fn recurse(node: &Rc<RefCell<DOMNode>>, style: &Style) -> Vec<impl Widget<()>> {
    match &node.borrow().data {
        Data::Text(text) => {
            let mut body = Vec::new();
            body.push(label(&text.text, style)); 
            return body;
        },
        Data::Element(elem) => { 
            let style = open_tag(&elem.tag, style);
            let mut body = Vec::new();
            for child in &*node.borrow().children.borrow() {
                for label in recurse(&Rc::clone(child), &style) {
                    body.push(label);
                }
            }
            return body;
        }
    }    
}

pub fn layout(node: &Rc<RefCell<DOMNode>>, style: &Style) -> Rc<LayoutNode> {
    return Rc::new(LayoutNode::new(node, false));

    // for child in &*node.borrow().children.borrow() {
    //     layout(child, style);
    // }


    // match &node.borrow().data {
    //     Data::Text(text) => { 
    //         println!("{}", text.text);
    //     },
    //     Data::Element(elem) => { 
    //         println!("<{}>", elem.tag);
    //         for child in &*node.borrow().children.borrow() {
    //             print_dom(child, indent + 2);
    //         }
    //         print_indent(indent);
    //         println!("</{}>", elem.tag);
    //     }
    // }
}

pub fn render(node: &Rc<LayoutNode>) { }

// class DocumentLayout:
//     def __init__(self, node):
//         self.node = node
//         self.parent = None
//         self.children = []

//     def layout(self):
//         child = BlockLayout(self.node, self, None)
//         self.children.append(child)
//         child.layout()

// class BlockLayout:
//     def __init__(self, node, parent, previous):
//         self.node = node
//         self.parent = parent
//         self.previous = previous
//         self.children = []

//     def layout(self):
//         previous = None
//         for child in self.node.children:
//             if layout_mode(child) == "inline":
//                 next = InlineLayout(child, self, previous)
//             else:
//                 next = BlockLayout(child, self, previous)
//             self.children.append(next)
//             previous = next
//         for child in self.children:
//             child.layout()

// class InlineLayout:
//     def __init__(self, node, parent, previous):
//         self.node = node
//         self.parent = parent
//         self.previous = previous
//         self.children = []

// def layout_mode(node):
//     if isinstance(node, Text):
//         return "inline"
//     elif node.children:
//         for child in node.children:
//             if isinstance(child, Text): continue
//             if child.tag in BLOCK_ELEMENTS:
//                 return "block"
//         return "inline"
//     else:
//         return "block"