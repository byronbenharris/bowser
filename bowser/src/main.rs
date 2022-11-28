use druid::piet::InterpolationMode;
use druid::widget::{FillStrat, Flex, Image, Label, Scroll};
use druid::{AppLauncher, FontDescriptor, FontFamily, FontWeight, FontStyle, ImageBuf, LocalizedString, Widget, WindowDesc};
use std::cell::RefCell;
use std::{env, str};
use std::rc::Rc;

use bowser::html::{DOMNode, Data, print_dom, parse};
use bowser::request::request;

#[derive(Debug)]
struct Style {
    size: f64, 
    bold: bool, 
    italic: bool,
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
            let dom_root = parse(&body_str.to_string());
            print_dom(&Rc::clone(&dom_root), 0);
            // let layout_root = parse_layout(&Rc::clone(&dom_root), &Style { size: 16.0, bold: false, italic: false });
            // draw_layout(&Rc::clone(&layout_root));
            let body_widgets = recurse(
                &Rc::clone(&dom_root), &Style { size: 16.0, bold: false, italic: false });
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

// fn parse_layout(node: &Rc<RefCell<DOMNode>>, style: &Style) -> LayoutNode {

// }

fn recurse(node: &Rc<RefCell<DOMNode>>, style: &Style) -> Vec<impl Widget<()>> {
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
