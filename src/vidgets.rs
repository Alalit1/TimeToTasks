use fltk::{button::Button, prelude::*};
use fltk::enums::Color;
use fltk::enums::FrameType;

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
