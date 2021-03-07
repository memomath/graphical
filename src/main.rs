use fltk::*;

fn create_window() {
    //constants
    let app = app::App::default();
    let win_width = 500;
    let win_height = 500;

    let mut win = window::Window::new(100, 100, win_width, win_height, "Mathical Graph Engine");
    win.draw(move || {
        draw::set_draw_color(Color::Black);
        draw::draw_line(win_width / 2, 0, win_width / 2, win_height);
        draw::draw_line(0, win_height / 2, win_width, win_height / 2);
    });

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