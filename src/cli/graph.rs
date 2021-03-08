use fltk::*;


fn functioner(x:i32) -> i32{
    return (x * x) / 80; // f(x)=x
}

//construct window and graph function
pub fn graph() {
    //constants
    let app = app::App::default();
    let win_width = 500;
    let win_height = 500;

    let num_of_lines = 50;
    let num_of_places = win_width / num_of_lines;

    let mut win = window::Window::new(100, 100, win_width, win_height, "Mathical Graph Engine");

    win.draw(move || {
        // grid
        draw::set_draw_color(Color::White);
        for i in 1..num_of_lines {
            draw::draw_line(i * num_of_places, 0, i * num_of_places, 500);
            draw::draw_line(0, i * num_of_places, 500, i * num_of_places); 
        }

        //lines
        draw::set_draw_color(Color::Black);
        draw::draw_line(win_width / 2, 0, win_width / 2, win_height);
        draw::draw_line(0, win_height / 2, win_width, win_height / 2);

        //thicker lines
        draw::draw_line(win_width / 2 + 1, 0, win_width / 2 + 1, win_height);
        draw::draw_line(0, win_height / 2 + 1, win_width, win_height / 2 + 1);
        draw::draw_line(win_width / 2 - 1, 0, win_width / 2 - 1, win_height);
        draw::draw_line(0, win_height / 2 - 1, win_width, win_height / 2 - 1);

        draw::set_draw_color(Color::Blue);

        for i in -250..251 {
            let y = functioner(i);
            draw::draw_point(i + 250, -y + 250);
        }
        
    });

    win.end();
    win.show();
    app.run().unwrap();
}