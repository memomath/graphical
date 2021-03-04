use std::env;

fn process_input(){
    let args: Vec<String> = env::args().collect();

    if args.len() < 4 {
        eprintln!("Missing some values");
        std::process::exit(0x001);
    }

    let first_argument: &str = &*args[1].to_lowercase();
    let second_argument: f64 = args[2].parse().unwrap();
    let third_argument: f64 = args[3].parse().unwrap();

    match first_argument {
        "add" => {
            println!("{}", second_argument + third_argument);
        }

        "subtract" => {
            println!("{}", second_argument - third_argument);
        }

        "multiply" => {
            println!("{}", second_argument * third_argument);
        }
        "divide" => {

            println!("{}", second_argument / third_argument);
        }

        "mod" => {
            println!("{}", second_argument % third_argument);
        }

        "--help" => {
            println!("Help");
        }
        _ => {
            println!("Unknown command");
        }
    }
}
fn main() {
    process_input();
}
