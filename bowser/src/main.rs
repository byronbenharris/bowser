use druid::piet::InterpolationMode;
use druid::widget::{FillStrat, Flex, Image, Label, Scroll};
use druid::{AppLauncher, ImageBuf, Widget, WindowDesc};
use std::{env, str};
use std::rc::Rc;

use bowser::html::{print_dom, parse};
use bowser::layout::{Style, recurse, layout, render};
use bowser::request::request;

fn load(url: &String) -> impl Widget<()> {
    let (headers, body) = request(url);
    assert!(headers.contains_key("content-type"));
    let content_type = headers.get("content-type").unwrap().splitn(2, ';').next().unwrap();
    let mut col = Flex::column();
    match content_type {
        "text/html" => {
            let body_str = str::from_utf8(&body).expect("Failed to convert [u8] to string");
            let dom_root = parse(&body_str.to_string());
            print_dom(&Rc::clone(&dom_root), 0);
            // let layout_root = layout(&Rc::clone(&dom_root), &Style::new());
            // render(&Rc::clone(&layout_root));
            let body_widgets = recurse(&Rc::clone(&dom_root), &Style::new());
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

fn main() {
    let args: Vec<String> = env::args().collect();
    let url = &args[1];
    AppLauncher::with_window(
        WindowDesc::new(load(url)).title(String::from("Bowser"))
    ).launch(()).expect("failed to launch gui");
}
