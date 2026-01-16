use fltk::{button::Button, prelude::*};
use fltk::enums::Color;
use fltk::enums::FrameType;
use fltk::{
    app, window::Window, frame::Frame,
    app::Sender, app::Receiver, app::channel,
};

pub struct ButtonConfig<'a> {
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
    pub label: &'a str,
}

pub fn create_button<F>(
    cfg: ButtonConfig,
    mut callback: F,
) -> Button
where
    F: FnMut() + 'static,
{
    let mut btn = Button::new(cfg.x, cfg.y, cfg.w, cfg.h, cfg.label);
    
    btn.set_color(Color::from_rgb(24, 24, 27));
    btn.set_label_color(Color::from_rgb(255, 255, 255));
    btn.set_callback(move |_| callback());
    btn.set_frame(FrameType::RoundedBox);
    btn
}


#[derive(Clone, Copy)]
pub enum MsgResult {
    Ok,
    Cancel,
}

pub fn message_confirm(text: &str) -> MsgResult {
    let (s, r): (Sender<MsgResult>, Receiver<MsgResult>) = channel();

    let mut win = Window::new(400, 300, 360, 160, "Підтвердження");
    win.set_color(Color::from_rgb(30, 30, 30));

    let mut label = Frame::new(20, 30, 320, 60, text);
    label.set_label_color(Color::White);

    let mut ok = Button::new(60, 100, 100, 35, "OK");

    ok.set_callback(move |_| s.send(MsgResult::Ok));
    cancel.set_callback(move |_| s.send(MsgResult::Cancel));

    win.end();
    win.show();

    while let Some(msg) = r.recv() {
        win.hide();
        return msg;
    }

    MsgResult::Cancel
}

use fltk::{
    input::Input,
    enums::{Color, FrameType},
};

pub struct InputConfig<'a> {
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
    pub label: &'a str,
    pub placeholder: &'a str,
}
pub fn create_input(cfg: InputConfig) -> Input {
    let mut input = Input::new(
        cfg.x,
        cfg.y,
        cfg.w,
        cfg.h,
        cfg.label,
    );

    // Placeholder (підказка в полі)
    input.set_value(cfg.placeholder);

    // Стиль
    input.set_color(Color::from_rgb(24, 24, 27));
    input.set_text_color(Color::White);
    input.set_frame(FrameType::RoundedBox);
    input.set_text_size(14);

    input
}
