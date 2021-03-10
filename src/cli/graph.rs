use fltk::*;

fn functioner(mut x:f64) -> f64 {
    x = x/(5 as f64);
    return x * x * x;
    //return x.cos() * 10 as f64;
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

        let mut last_x = 0;
        let mut last_y = 0;
        let mut set = false;

        for x in -250 * 100/5..251 * 100/5 {
            let x = x as f64 * 0.05;
            let y = -((functioner(x as f64) * 8 as f64) as i32) + 250;
            let x = (x * 8 as f64) as i32 + 250;

            if set {
                draw::draw_line(last_x,last_y,x,y);
                last_x = x;
                last_y = y;
            } else {
                last_x = x;
                last_y = y;
                set = true;
            }
        }
    });
    win.end();
    win.show();
    app.run().unwrap();
}
