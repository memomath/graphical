pub mod graph;

pub mod cli {
    use super::graph;

    //help message
    pub fn help() {
        println!("no help for you")
    }

    pub fn cli_init() {
        let args: Vec<String> = std::env::args().collect();
        let first_argument: &str = &*args[1].to_lowercase();

        match first_argument {
            "graph" => {
                graph::graph();
            }
            
            "help" => {
                help();
            }
            _ => {
                println!("Unknown command");
            }
        }
    }
}
