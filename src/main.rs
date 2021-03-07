use fltk::{app, window::*, button::*};

fn create_window() {
    let app = app::App::default();
    let mut win = Window::new(100, 100, 400, 300, "Mathical Graph Engine");
    let but1 = Button::new(10, 10, 80, 40, "Button 1");

    win.end();
    win.show();
    app.run().unwrap();
}

fn process_input() {
    let args: Vec<String> = std::env::args().collect();
    
    let first_argument: &str = &*args[1].to_lowercase();

    match first_argument {
        "graph" => {
            create_window()
        }

        _ => {
            println!("Unknown command");
        }
    }
}
fn main() {
    process_input();
} 