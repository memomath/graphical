pub mod graph;
mod parser;

pub mod cli {
    use colored::Colorize;

    use super::{graph, parser};

    fn functioner(mut x: f64) -> f64 {
        x = x / (5 as f64);
        return x * x * x;
        //return x.cos() * 10 as f64;
    }

    pub fn init() {
        let mut args: Vec<String> = std::env::args().collect();

        match args.get_mut(1) {
            Some(x) => match &*x.to_lowercase() {
                "graph" => {
                    println!("{}", "Successfully graphed!".green().bold());
                    graph::graph(functioner);
                }
                "--help" | "-h" => {
                    print_help_message();
                }

                "--version" | "-v" => {
                    print_version();
                }

                "--error-test" => {
                    error("Missing argument 1.", "mathical --help");
                }

                _ => {
                    println!("{}: Unknown command", "ERROR:".red());
                }
            },
            None => {
                println!("{}", "Need first argument".red().bold());
                std::process::exit(0x0001);
            }
        }

        let _parsed_equation: () = parser::parser::parse();
    }

    fn print_help_message() {
        let help_message: &str = "
Mathical Version 1.0.0

USAGE:
mathical <action> [arguments]

FLAGS:
    -h, --help       Prints help information
    -v, --version    Prints version information

ACTIONS:
    graph <args>     Creates a window with graph of the function or equation provided

ARGS:
    <action>         The type of action performed
    <arguments>      Arguments to the action
";

        return println!("{}", &help_message);
    }

    fn print_version() {
        let version: &str = "v1.0.0";

        return println!("{}", &version);
    }

    pub fn error(error_message: &str, usage: &str) {
        let colored_error: &str = &"ERROR:";

        return println!(
            "
{} {}

USAGE:
    {}

For more information, try the command {}
",
            colored_error.red().bold(),
            error_message,
            usage,
            "--help".green().bold()
        );
    }
}
