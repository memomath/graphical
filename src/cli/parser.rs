enum Grammar {
    Add,
    Subtract,
    Divide,
    Multiply,
    Exponent
}

struct ParseNode {
    children: Vec<ParseNode>,
    entry: Grammar,
}
pub mod parser {
    pub fn parse() {

    }
}