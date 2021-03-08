pub mod graph;

pub mod cli {
    use colored::Colorize;

    use super::graph;

    pub fn cli_init() {
        let args: Vec<String> = std::env::args().collect();
        let first_argument: &str = &*args[1].to_lowercase();

        match first_argument {
            "graph" => {
                println!("{}", "Successfully graphed!".green().bold());
                graph::graph();
            }
            _ => {
                println!("{}: Unknown command", "ERROR:".red());
            }
        }
    }
}
