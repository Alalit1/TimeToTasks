use fltk::{prelude::*, group::Group};
use fltk::app;

pub struct ScreenManager {
    pub width: u16,
    pub height: u16,
    // додаємо зберігання активних груп
    pub current: Option<Group>,
}

impl ScreenManager {
    pub fn draw(&mut self) {
        let app = fltk::app::App::default();
        let mut wind = fltk::window::Window::new(100, 100, self.width.into(), self.height.into(), "SkillRoots");
        wind.begin();
        // Завантажуемо іконку для вікна
        let icon = PngImage::load("Icon_SkillRoots.png").unwrap();

        // Встановлюємо іконку вікна
        wind.set_icon(Some(icon));

        // --- Створюємо всі екрани (групи) через твої build_*_menu ---
        let login_menu = build_login_menu({
            let wind_clone = wind.clone();
            move |action| {
                match action {
                    Action::GoMain => {
                        // приховуємо login
                        wind_clone.hide(); // або login_menu.hide() якщо зберігаєш Group
                        // показуємо main
                    }
                    _ => {}
                }
            }
        });

        let main_menu = build_main_menu({
            let wind_clone = wind.clone();
            move |action| match action {
                Action::Start => println!("Старт"),
                Action::Exit => fltk::app::quit(),
                _ => {}
            },
        });

        // Спочатку показуємо login, приховуємо main
        main_menu.hide();
        self.current = Some(login_menu.clone());

        wind.end();
        wind.show();
        app.run().unwrap();
    }

    // метод для перемикання екранів
    pub fn switch_screen(&mut self, next: Group) {
        if let Some(curr) = &self.current {
            curr.hide();
        }
        next.show();
        self.current = Some(next);
    }
}
/*
let login_menu_group = build_login_menu(|action| match action {
    Action::Start => {
        println!("Натиснуто Почати");
        // тут твоя логіка запуску задачі
    },
    Action::Exit => {
        println!("Натиснуто Вийти");
        app::quit(); // або будь-який інший код для виходу
    },
    _ => {}
});*/

// Структура для керування станом програми
/*#[derive(Clone, Copy, PartialEq)]
pub enum WindowType {

}
#[derive(Copy, Clone, Debug)]
pub enum Action {

}

ui::menu(&mut wind);

*/