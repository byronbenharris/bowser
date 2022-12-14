use druid::piet::InterpolationMode;
use druid::widget::{prelude::*, Button, FillStrat, Flex, Image, Label, Scroll, TextBox};
use druid::{AppLauncher, ImageBuf, Widget, WidgetExt, WindowConfig, WindowDesc, WindowLevel};
use std::rc::Rc;
use std::{env, str, fs};
// use std::sync::Arc;

use bowser::html::{parse, print_dom};
use bowser::layout::{recurse, AppState, Style};
use bowser::request::request;

fn load(url: &String) -> impl Widget<AppState> {
    let (headers, body) = request(url);
    assert!(headers.contains_key("content-type"));
    let content_type = headers
        .get("content-type")
        .unwrap()
        .splitn(2, ';')
        .next()
        .unwrap();
    let mut col = Flex::column().cross_axis_alignment(druid::widget::CrossAxisAlignment::Start);
    let content = match content_type {
        "text/html" => {
            let body_str = str::from_utf8(&body).expect("Failed to convert [u8] to string");
            let dom_root = parse(&body_str.to_string());
            print_dom(&Rc::clone(&dom_root), 0);
            // let layout_root = layout(&Rc::clone(&dom_root), &Style::new());
            // render_page(&Rc::clone(&layout_root))
            let body_widgets = recurse(&Rc::clone(&dom_root), &Style::new());
            for widget in body_widgets {
                col.add_child(widget);
            }
            col
        }
        "image/png" => {
            let img_data =
                ImageBuf::from_data(&body).expect("Failed to store bytes in image buffer");
            let img = Image::new(img_data)
                .fill_mode(FillStrat::Fill)
                .interpolation_mode(InterpolationMode::Bilinear);
            col.with_child(img)
        }
        _ => col.with_child(Label::new("Unknown content type")),
    };
    return Scroll::new(content).vertical();
}

fn build_root_widget() -> impl Widget<AppState> {
    let img_buf = fs::read("bowser.png").expect("Failed to read contents of file");
    let img_data = ImageBuf::from_data(&img_buf).expect("Failed to store bytes in image buffer");
    return Flex::column()
        .with_default_spacer()
        .cross_axis_alignment(druid::widget::CrossAxisAlignment::Start)
        .with_child(
            Image::new(img_data)
                .fill_mode(FillStrat::Fill)
                .interpolation_mode(InterpolationMode::Bilinear)
                .fix_size(300., 300.)
                .center()
        )
        .with_default_spacer()
        .with_child(
            Label::new("Welcome to Bowser!")
            .center()
        )
        .with_default_spacer()
        .with_child(
            Flex::row()
                .with_child(
                    TextBox::new().lens(AppState::url)
                )
                .with_child(
                    Button::new("Go").on_click(|ctx, state: &mut AppState, env| {
                        let page = load(&state.url);
                        ctx.new_sub_window(
                            WindowConfig::default()
                                .show_titlebar(false)
                                .window_size(Size::new(500., 500.))
                                .set_level(WindowLevel::AppWindow),
                            page,
                            state.clone(),
                            env.clone(),
                        );
                    }),
                )
                .center()
        );
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let url = if args.len() <= 1 {
        String::from("")
    } else {
        args[1].to_string()
    };

    let state = AppState { url };
    let window = WindowDesc::new(
        build_root_widget()).title(String::from("Bowser")
    );
    AppLauncher::with_window(window)
        .launch(state)
        .expect("failed to launch gui");
}
