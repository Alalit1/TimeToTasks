mod vidgets;
use fltk::app::App;
fn main() {
    // передача меню 
    let app = App::default();
    vidgets::message_confirm("Це мій власний message box");
    app.run().unwrap();
}