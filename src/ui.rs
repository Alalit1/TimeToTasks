use fltk::{frame::Frame, group::Group, prelude::*};
use crate::screenmanager::Action;
use crate::vidgets::{ButtonConfig, create_button};

pub struct ButtonDef<'a> {
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
    pub label: &'a str,
    pub action: Action,
}

pub fn build_main_menu(
    on_action: impl Fn(Action) + Clone + 'static,
) -> Group {
    let group = Group::new(0, 0, 400, 300, "");
    Frame::new(0, 0, 400, 200, "Це головне меню");

    let buttons = vec![
        ButtonDef {
            x: 50, y: 50, w: 120, h: 35,
            label: "Start",
            action: Action::Start,
        },
        ButtonDef {
            x: 50, y: 95, w: 120, h: 35,
            label: "Settings",
            action: Action::Settings,
        },
        ButtonDef {
            x: 50, y: 140, w: 120, h: 35,
            label: "Exit",
            action: Action::Exit,
        },
    ];

    for b in buttons {
        let on_action = on_action.clone();
        let action = b.action;
        
        create_button(
            ButtonConfig {
                x: b.x,
                y: b.y,
                w: b.w,
                h: b.h,
                label: b.label,
            },
            move || on_action(action)
        );
    }

    group
}

pub fn build_settings(
    on_action: impl Fn(Action) + Clone + 'static,
) -> Group {
    let group = Group::new(0, 0, 400, 300, "");
    Frame::new(0, 0, 400, 200, "Налаштування");

    let on_action_clone = on_action.clone();
    
    create_button(
        ButtonConfig {
            x: 50,
            y: 50,
            w: 120,
            h: 35,
            label: "Назад",
        },
        move || on_action_clone(Action::Back)
    );

    group
}