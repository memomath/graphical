pub mod graph;

pub mod cli {
    use super::graph;

    pub fn cli_init() {
        let args: Vec<String> = std::env::args().collect();
        let first_argument: &str = &*args[1].to_lowercase();

        match first_argument {
            "graph" => {
                graph::graph();
            }
            _ => {
                println!("Unknown command");
            }
        }
    }
}
