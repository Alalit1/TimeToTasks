use fltk::{prelude::*, group::Group};
use fltk::app;

#[derive(Copy, Clone, Debug)]
pub enum Action {
    Start,
    Settings,
    Exit,
    Back,
    Mash,
    Typing,
    Language,
}

#[derive(Clone, Copy, PartialEq)]
pub enum WindowType {
    MainMenu,
    Settings,
    Mash,
    Typing,
    Language,
}

// Структура для керування станом програми
pub struct AppState {
    current_window: WindowType,
    content_group: Group,
}

impl AppState {
    pub fn new(main_window: &mut fltk::window::Window) -> Self {
        let content_group = Group::new(0, 0, 400, 300, "");
        main_window.add(&content_group);
        
        Self {
            current_window: WindowType::MainMenu,
            content_group,
        }
    }
    
    pub fn handle_action(&mut self, action: Action) {
        match action {
            Action::Start => {
                println!("Запуск гри...");
            },
            Action::Settings => {
                self.show_window(WindowType::Settings);
            },
            Action::Mash => {
                self.show_window(WindowType::Mash);
            },
            Action::Typing => {
                self.show_window(WindowType::Typing);
            },
            Action::Language => {
                self.show_window(WindowType::Language);
            },
            Action::Back => {
                self.show_window(WindowType::MainMenu);
            },
            Action::Exit => {
                println!("Вихід з програми");
                // Вихід обробляється в main.rs
            },
        }
    }
    
    fn show_window(&mut self, window_type: WindowType) {
        self.current_window = window_type;
        self.redraw_current_window();
    }
    
    pub fn redraw_current_window(&mut self) {
        // 1. Очищаємо старий вміст
        self.content_group.clear();
        
        // 2. Створюємо новий вміст залежно від поточного вікна
        let label = match self.current_window {
            WindowType::MainMenu => "Головне меню",
            WindowType::Settings => "Налаштування",
            WindowType::Mash => "Гра MASH",
            WindowType::Typing => "Тренування друку",
            WindowType::Language => "Мова",
        };
        
        fltk::frame::Frame::new(0, 0, 400, 200, label)
            .with_align(fltk::enums::Align::Center);
    }
}