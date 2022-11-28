use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

use crate::html::{DOMNode, Data};

#[derive(Debug)]
pub struct LayoutNode {
    pub node: DOMNode,
    pub inline: bool,
    pub children: RefCell<Vec<Rc<LayoutNode>>>,
}

const BLOCK_ELEMENTS: [&str; 37] = [
    "html", "body", "article", "section", "nav", "aside",
    "h1", "h2", "h3", "h4", "h5", "h6", "hgroup", "header",
    "footer", "address", "p", "hr", "pre", "blockquote",
    "ol", "ul", "menu", "li", "dl", "dt", "dd", "figure",
    "figcaption", "main", "div", "table", "form", "fieldset",
    "legend", "details", "summary"
];

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