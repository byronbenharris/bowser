use druid::piet::InterpolationMode;
use druid::widget::{FillStrat, Flex, Image, Label, Scroll, TextBox};
use druid::{AppLauncher, ImageBuf, Widget, WindowDesc, WidgetExt, Lens, Data};
use std::{env, str};
use std::rc::Rc;
// use std::sync::Arc;

use bowser::html::{print_dom, parse};
use bowser::layout::{Style, recurse, layout, render};
use bowser::request::request;

// running the endokernel in rust is hard
// if our final result is we failed to do so... 
// is that okay if we describe what we tried

// #[derive(Clone, Data, Lens)]
// struct State {
//     url: String,
// }

fn load(url: &String) -> impl Widget<()> {
    let (headers, body) = request(url);
    assert!(headers.contains_key("content-type"));
    let content_type = headers.get("content-type").unwrap().splitn(2, ';').next().unwrap();
    let content: Flex<()> = match content_type {
        "text/html" => {
            let body_str = str::from_utf8(&body).expect("Failed to convert [u8] to string");
            let dom_root = parse(&body_str.to_string());
            print_dom(&Rc::clone(&dom_root), 0);
            // let layout_root = layout(&Rc::clone(&dom_root), &Style::new());
            // render_page(&Rc::clone(&layout_root))
            let mut col = Flex::column();
            // col.add_child(TextBox::new().with_placeholder(*url).lens(State::url));
            let body_widgets = recurse(&Rc::clone(&dom_root), &Style::new());
            for widget in body_widgets {
                col.add_child(widget);
            }
            col
        },
        "image/png" => {
            let img_data = ImageBuf::from_data(&body)
                .expect("Failed to store bytes in image buffer");
            let img = Image::new(img_data)
                .fill_mode(FillStrat::Fill)
                .interpolation_mode(InterpolationMode::Bilinear);
            Flex::column().with_child(img)
        },
        _ => Flex::column().with_child(Label::new("Unknown content type")),  
    };
    return Scroll::new(content).vertical();
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let url = &args[1];

    // let initial_state = State { url: "".to_string().into() };
    let window = WindowDesc::new(load(url)).title(String::from("Bowser"));
    AppLauncher::with_window(window).launch(()).expect("failed to launch gui");
}
