use fltk::{prelude::*, *};
use std::rc::Rc;
use crate::app;
use fltk::group::Group;
use std::cell::RefCell;

mod screenmanager;
mod ui;
mod vidgets;  // Додайте цей рядок перед використанням


use screenmanager::{AppState, WindowType, Action};
use ui::{build_main_menu, build_settings};

fn main() {
    let app = app::App::default();

    let mut win = Window::new(100, 100, 400, 300, "App");

    let mut menu = Group::new(0, 0, 400, 300, "");
    let mut btn = Button::new(150, 130, 100, 40, "Start");
    menu.end();

    let mut game = Group::new(0, 0, 400, 300, "");
    game.hide(); // спочатку приховане
    game.end();

    win.end();
    win.show();

    btn.set_callback(move |_| {
        menu.hide();
        game.show();
    });

    app.run().unwrap();
}