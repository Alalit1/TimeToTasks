use fltk::{frame::Frame, group::Group, prelude::*};
use crate::screenmanager::Action;
use crate::vidgets::{ButtonConfig, create_button};


pub fn build_login_menu(
    on_action: impl Fn(Action) + Clone + 'static,
) -> Group {
    let mut group = Group::new(0, 0, 400, 300, "");
    group.begin();

    Frame::new(0, 20, 400, 40, "Це головне меню");

    let buttons = vec![
        ButtonConfig {
            x: 100,
            y: 100,
            w: 200,
            h: 40,
            label: "Почати",
            action: Action::Start,
        },
        ButtonConfig {
            x: 100,
            y: 160,
            w: 200,
            h: 40,
            label: "Вийти",
            action: Action::Exit,
        },
    ];

    for b in buttons {
        let on_action = on_action.clone();
        let action = b.action;

        create_button(b, move || on_action(action));
    }

    group.end();
    group
}

pub fn build_main_menu(
    on_action: impl Fn(Action) + Clone + 'static,
) -> Group {
    let group = Group::new(0, 0, 400, 300, "");
    Frame::new(0, 0, 400, 200, "Це головне меню");

    //
    let buttons = vec![
        ButtonConfig {
                x: b.x,
                y: b.y,
                w: b.w,
                h: b.h,
                label: b.label,
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